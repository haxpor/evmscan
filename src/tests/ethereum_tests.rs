/**
 * Note that test module needs etherscan.io's API key in which it needs to be
 * defined via environment variable namely 'EVMSCAN_TEST_ETHERSCAN_APIKEY'.
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
    // as imposed by etherscan.io.
    static ref LOCK: Mutex<()> = Mutex::new(());

    // target address which is "Project Wyvern Exchange of Opensea"
    static ref ADDRESS1: &'static str = "0x7Be8076f4EA4A4AD08075C2508e481d6C946D12b";

    // another target address which is "Tornado.Cash: Proxy"
    static ref ADDRESS2: &'static str = "0x722122dF12D4e14e13Ac3b6895a86e84145b6967";
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

/// This function will panic if EVMSCAN_TEST_ETHERSCAN_APIKEY is not defined.
fn get_api_key_or_panic() -> String {
    env::var("EVMSCAN_TEST_ETHERSCAN_APIKEY").expect("Error: define 'EVMSCAN_TEST_ETHERSCAN_APIKEY' environment variable for testing")
}

serial_test! {
    fn test_get_balance() {
        let ctx = Context::create(ChainType::Ethereum, get_api_key_or_panic());

        let _balance = evmscan::accounts().get_balance_address(&ctx, &ADDRESS1).unwrap();
    }
}

serial_test! {
    fn test_get_balance_multi() {
        let ctx = Context::create(ChainType::Ethereum, get_api_key_or_panic());

        let txs = evmscan::accounts().get_balance_addresses_multi(&ctx, &[&ADDRESS1, &ADDRESS2]).unwrap();
        assert!(txs.len() == 2);
    }
}

// NOTE: only downside here is the time it takes to wait for response
// as it will max out at 10000 which is the maximum limitation that this
// API can return.
serial_test! {
    fn test_get_list_normal_txs() {
        let ctx = Context::create(ChainType::Ethereum, get_api_key_or_panic());

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
        let ctx = Context::create(ChainType::Ethereum, get_api_key_or_panic());

        let txs = evmscan::accounts().get_list_internal_transactions(&ctx, &ADDRESS1).unwrap();
        assert!(txs.len() == 10000);
    }
}

// TODO: separate this Stats related API into separate test file
serial_test! {
    fn test_get_native_token_last_price() {
        let ctx = Context::create(ChainType::Ethereum, get_api_key_or_panic());

        match evmscan::stats().get_native_token_last_price(&ctx) {
            Ok(res) => println!("{:#?}", res),
            Err(e) => panic!("{:?}:", e)
        }
    }
}

// TODO: separate this Contracts related API into seperate test file
serial_test! {
    fn test_contracts_get_abi_with_no_pretty_print() {
        let ctx = Context::create(ChainType::Ethereum, get_api_key_or_panic());

        let res = evmscan::contracts().get_abi(&ctx, &ADDRESS1, false);
        assert!(res.is_ok());
        assert!(res.unwrap().len() == 12500);   // exact number of character
                                                // from cleaned '\' char
    }
}

serial_test! {
    fn test_contracts_get_abi_with_pretty_print() {
        let ctx = Context::create(ChainType::Ethereum, get_api_key_or_panic());

        let res = evmscan::contracts().get_abi(&ctx, &ADDRESS1, true);
        assert!(res.is_ok());
        assert!(res.unwrap().len() > 12500);
    }
}

serial_test! {
    fn test_contracts_get_verified_source_code() {
        let ctx = Context::create(ChainType::Ethereum, get_api_key_or_panic());

        // this contract address has informatin in 'implementation' field, and
        // acts as a proxy, as well as has at least one constructor argument.
        match evmscan::contracts().get_verified_source_code(&ctx, "0xAfC2F2D803479A2AF3A72022D54cc0901a0ec0d6") {
            Err(e) => panic!("{:?}", e),
            Ok(res) => {
                assert!(res.0.len() > 0);
                assert_eq!(res.1, false);       // not submitted as JSON format

                assert_eq!(res.0[0].constructor_arguments.len(), 1);
                assert_eq!(res.0[0].constructor_arguments[0], "000000000000000000000000b6029ea3b2c51d09a50b53ca8012feeb05bda35a");

                assert_eq!(res.0[0].optimization_used, false);
                assert_eq!(res.0[0].compiler_version, "v0.5.7+commit.6da8b019");
                assert_eq!(res.0[0].runs, 200);
                assert_eq!(res.0[0].contract_name, "Proxy");
                assert_eq!(res.0[0].evm_version, "Default");
                assert_eq!(res.0[0].license_type, "");
                assert_eq!(res.0[0].proxy, true);
                assert_eq!(res.0[0].implementation, "0x34cfac646f301356faa8b21e94227e3583fe3f5f");
                assert_eq!(res.0[0].swarm_source, "bzzr://1e7d648b83cfac072cbccefc2ffc62a6999d4a050ee87a721942de1da9670db8");
            }
        }
    }
}

serial_test! {
    // The verification step can be in json format
    // https://docs.soliditylang.org/en/v0.5.8/using-the-compiler.html#compiler-input-and-output-json-description
    // thus it can contain multiple files with the optional of settings.
    fn test_contracts_get_verified_source_code_json_format() {
        let ctx = Context::create(ChainType::Ethereum, get_api_key_or_panic());

        // ALERT: this contract address is taken for example in this test case,
        // if you interact with it, please be careful and vigilant. Source:
        // https://twitter.com/PeckShieldAlert/status/1489467040419258370?s=20&t=6j7XEjRChV8lrcojbbKWzg
        match evmscan::contracts().get_verified_source_code(&ctx, "0xa5DEf515cFd373D17830E7c1de1639cB3530a112") {
            Err(e) => panic!("{}", e),
            Ok(res) => {
                assert_eq!(res.0.len(), 11);     // 1 + 10 (1 is raw combined altogether, and 10 is other files there as part of JSON format)

                assert_eq!(res.1, true);    // this is submitted as JSON format, so it's true

                assert_eq!(res.0[0].constructor_arguments.len(), 0);
                assert_eq!(res.0[0].compiler_version, "v0.8.4+commit.c7e474f2");
                assert_eq!(res.0[0].runs, 999999);
                assert_eq!(res.0[0].evm_version, "Default");
                assert_eq!(res.0[0].license_type, "");
                assert_eq!(res.0[0].proxy, false);
                assert_eq!(res.0[0].implementation, "");
                assert_eq!(res.0[0].swarm_source, "");
                assert_eq!(res.0[0].contract_name, "DePoToken");

                assert_eq!(res.0[1].contract_name, "contracts/DePoToken.sol");
                assert_eq!(res.0[2].contract_name, "contracts/UsingLiquidityProtectionService.sol");
                assert_eq!(res.0[3].contract_name, "@openzeppelin/contracts/access/Ownable.sol");
                assert_eq!(res.0[4].contract_name, "@openzeppelin/contracts/token/ERC20/ERC20.sol");
                assert_eq!(res.0[5].contract_name, "contracts/external/UniswapV2Library.sol");
                assert_eq!(res.0[6].contract_name, "contracts/external/UniswapV3Library.sol");
                assert_eq!(res.0[7].contract_name, "contracts/IPLPS.sol");
                assert_eq!(res.0[8].contract_name, "@openzeppelin/contracts/utils/Context.sol");
                assert_eq!(res.0[9].contract_name, "@openzeppelin/contracts/token/ERC20/IERC20.sol");
                assert_eq!(res.0[10].contract_name, "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol");
            }
        }
    }
}
