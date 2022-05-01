/**
 * Note that test module needs etherscan.io's API key in which it needs to be
 * defined via environment variable namely 'EVMSCAN_TEST_POLYGONSCAN_APIKEY'.
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
    // as imposed by polygonscan.com.
    static ref LOCK: Mutex<()> = Mutex::new(());

    // target address which is "ETHTornado"
    static ref ADDRESS1: &'static str = "0x1E34A77868E19A6647b1f2F47B51ed72dEDE95DD";

    // another target address which is "SwapRouter"
    static ref ADDRESS2: &'static str = "0xE592427A0AEce92De3Edee1F18E0157C05861564";
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

/// This function will panic if EVMSCAN_TEST_POLYGONSCAN_APIKEY is not defined.
fn get_api_key_or_panic() -> String {
    env::var("EVMSCAN_TEST_POLYGONSCAN_APIKEY").expect("Error: define 'EVMSCAN_TEST_POLYGONSCAN_APIKEY' environment variable for testing")
}

serial_test! {
    fn test_get_balance() {
        let ctx = Context::create(ChainType::Polygon, get_api_key_or_panic());

        let _balance = evmscan::accounts().get_balance_address(&ctx, &ADDRESS1).unwrap();
    }
}

serial_test! {
    fn test_get_balance_multi() {
        let ctx = Context::create(ChainType::Polygon, get_api_key_or_panic());

        let txs = evmscan::accounts().get_balance_addresses_multi(&ctx, &[&ADDRESS1, &ADDRESS2]).unwrap();
        assert!(txs.len() == 2);
    }
}

// NOTE: only downside here is the time it takes to wait for response
// as it will max out at 10000 which is the maximum limitation that this
// API can return.
serial_test! {
    fn test_get_list_normal_txs() {
        let ctx = Context::create(ChainType::Polygon, get_api_key_or_panic());

        let txs = evmscan::accounts().get_list_normal_transactions(&ctx, &ADDRESS2).unwrap();

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
        let ctx = Context::create(ChainType::Polygon, get_api_key_or_panic());

        let txs = evmscan::accounts().get_list_internal_transactions(&ctx, &ADDRESS2).unwrap();
        assert!(txs.len() == 10000);
    }
}

// TODO: separate this Stats related API into separate test file
serial_test! {
    fn test_get_native_token_last_price() {
        let ctx = Context::create(ChainType::Polygon, get_api_key_or_panic());

        match evmscan::stats().get_native_token_last_price(&ctx) {
            Ok(res) => println!("{:#?}", res),
            Err(e) => panic!("{:?}:", e)
        }
    }
}

// TODO: separate this Contracts related API into seperate test file
serial_test! {
    fn test_contracts_get_abi_with_no_pretty_print() {
        let ctx = Context::create(ChainType::Polygon, get_api_key_or_panic());

        let res = evmscan::contracts().get_abi(&ctx, &ADDRESS1, false);
        assert!(res.is_ok());
        assert!(res.unwrap().len() == 5069);   // exact number of character
                                                // from cleaned '\' char
    }
}

serial_test! {
    fn test_contracts_get_abi_with_pretty_print() {
        let ctx = Context::create(ChainType::Polygon, get_api_key_or_panic());

        let res = evmscan::contracts().get_abi(&ctx, &ADDRESS1, true);
        assert!(res.is_ok());
        assert!(res.unwrap().len() > 5069);
    }
}

serial_test! {
    fn test_contracts_get_verified_source_code() {
        let ctx = Context::create(ChainType::Polygon, get_api_key_or_panic());

        match evmscan::contracts().get_verified_source_code(&ctx, &ADDRESS1) {
            Err(e) => panic!("{:?}", e),
            Ok(res) => {
                assert!(res.0.len() == 1);
                assert_eq!(res.1, false);       // not submitted as JSON format

                assert_eq!(res.0[0].constructor_arguments.len(), 4);
                assert_eq!(res.0[0].constructor_arguments[0], "000000000000000000000000fc9859303c0ac1a7721ece639f2e249d8fd72ac6");
                assert_eq!(res.0[0].constructor_arguments[1], "000000000000000000000000baffbe0e6c73d4dad3f813194695fdc5829c962a");
                assert_eq!(res.0[0].constructor_arguments[2], "0000000000000000000000000000000000000000000000056bc75e2d63100000");
                assert_eq!(res.0[0].constructor_arguments[3], "0000000000000000000000000000000000000000000000000000000000000014");

                assert_eq!(res.0[0].optimization_used, true);
                assert_eq!(res.0[0].compiler_version, "v0.7.6+commit.7338295f");
                assert_eq!(res.0[0].runs, 200);
                assert_eq!(res.0[0].contract_name, "ETHTornado");
                assert_eq!(res.0[0].evm_version, "Default");
                assert_eq!(res.0[0].license_type, "None");
                assert_eq!(res.0[0].proxy, false);
                assert_eq!(res.0[0].implementation, "");
                assert_eq!(res.0[0].swarm_source, "ipfs://8ceda25717bed5d981a763bf8fc532aaf1c7aafa06a512f58b473b522acadc99");
            }
        }
    }
}

serial_test! {
    // The verification step can be in json format
    // https://docs.soliditylang.org/en/v0.5.8/using-the-compiler.html#compiler-input-and-output-json-description
    // thus it can contain multiple files with the optional of settings.
    fn test_contracts_get_verified_source_code_json_format() {
        let ctx = Context::create(ChainType::Polygon, get_api_key_or_panic());

        // ALERT: this address we used has been attacked due to vulnerability in the migrate()
        // function (gymdefi) as reported by BlockSec
        // https://twitter.com/BlockSecTeam/status/1512832398643265537?s=20&t=n5hETJrbgTAANKTpiwiMeg.
        //
        // Such address is used in our test as it's not relatively easy to find
        // such contract that has submitted as part of code verification onto bscscan
        // that used JSON format which allows multiple files to be there.
        //
        // So be vigilant, and careful not to interact with such contract address.
        match evmscan::contracts().get_verified_source_code(&ctx, &ADDRESS2) {
            Err(e) => panic!("{}", e),
            Ok(res) => {
                assert_eq!(res.0.len(), 35);     // 1 + 34 (1 is raw combined altogether, and 34 is other files there as part of JSON format)

                assert_eq!(res.1, true);    // this is submitted as JSON format, so it's true

                assert_eq!(res.0[0].constructor_arguments.len(), 2);
                assert_eq!(res.0[0].constructor_arguments[0], "0000000000000000000000001f98431c8ad98523631ae4a59f267346ea31f984");
                assert_eq!(res.0[0].constructor_arguments[1], "0000000000000000000000000d500b1d8e8ef31e21c99d1db9a6444d3adf1270");
                assert_eq!(res.0[0].compiler_version, "v0.7.6+commit.7338295f");
                assert_eq!(res.0[0].optimization_used, true);
                assert_eq!(res.0[0].runs, 1000000);
                assert_eq!(res.0[0].evm_version, "Default");
                assert_eq!(res.0[0].license_type, "");
                assert_eq!(res.0[0].proxy, false);
                assert_eq!(res.0[0].implementation, "");
                assert_eq!(res.0[0].swarm_source, "");

                assert_eq!(res.0[1].contract_name, "contracts/SwapRouter.sol");
                assert_eq!(res.0[2].contract_name, "@uniswap/v3-core/contracts/libraries/SafeCast.sol");
                assert_eq!(res.0[3].contract_name, "@uniswap/v3-core/contracts/libraries/TickMath.sol");
                assert_eq!(res.0[4].contract_name, "@uniswap/v3-core/contracts/interfaces/IUniswapV3Pool.sol");
                assert_eq!(res.0[5].contract_name, "contracts/interfaces/ISwapRouter.sol");
                assert_eq!(res.0[6].contract_name, "contracts/base/PeripheryImmutableState.sol");
                assert_eq!(res.0[7].contract_name, "contracts/base/PeripheryValidation.sol");
                assert_eq!(res.0[8].contract_name, "contracts/base/PeripheryPaymentsWithFee.sol");
                assert_eq!(res.0[9].contract_name, "contracts/base/Multicall.sol");
                assert_eq!(res.0[10].contract_name, "contracts/base/SelfPermit.sol");
                assert_eq!(res.0[11].contract_name, "contracts/libraries/Path.sol");
                assert_eq!(res.0[12].contract_name, "contracts/libraries/PoolAddress.sol");
                assert_eq!(res.0[13].contract_name, "contracts/libraries/CallbackValidation.sol");
                assert_eq!(res.0[14].contract_name, "contracts/interfaces/external/IWETH9.sol");
                assert_eq!(res.0[15].contract_name, "@uniswap/v3-core/contracts/interfaces/pool/IUniswapV3PoolImmutables.sol");
                assert_eq!(res.0[16].contract_name, "@uniswap/v3-core/contracts/interfaces/pool/IUniswapV3PoolState.sol");
                assert_eq!(res.0[17].contract_name, "@uniswap/v3-core/contracts/interfaces/pool/IUniswapV3PoolDerivedState.sol");
                assert_eq!(res.0[18].contract_name, "@uniswap/v3-core/contracts/interfaces/pool/IUniswapV3PoolActions.sol");
                assert_eq!(res.0[19].contract_name, "@uniswap/v3-core/contracts/interfaces/pool/IUniswapV3PoolOwnerActions.sol");
                assert_eq!(res.0[20].contract_name, "@uniswap/v3-core/contracts/interfaces/pool/IUniswapV3PoolEvents.sol");
                assert_eq!(res.0[21].contract_name, "@uniswap/v3-core/contracts/interfaces/callback/IUniswapV3SwapCallback.sol");
                assert_eq!(res.0[22].contract_name, "contracts/interfaces/IPeripheryImmutableState.sol");
                assert_eq!(res.0[23].contract_name, "contracts/base/BlockTimestamp.sol");
                assert_eq!(res.0[24].contract_name, "@openzeppelin/contracts/token/ERC20/IERC20.sol");
                assert_eq!(res.0[25].contract_name, "@uniswap/v3-core/contracts/libraries/LowGasSafeMath.sol");
                assert_eq!(res.0[26].contract_name, "contracts/base/PeripheryPayments.sol");
                assert_eq!(res.0[27].contract_name, "contracts/interfaces/IPeripheryPaymentsWithFee.sol");
                assert_eq!(res.0[28].contract_name, "contracts/libraries/TransferHelper.sol");
                assert_eq!(res.0[29].contract_name, "contracts/interfaces/IPeripheryPayments.sol");
                assert_eq!(res.0[30].contract_name, "contracts/interfaces/IMulticall.sol");
                assert_eq!(res.0[31].contract_name, "@openzeppelin/contracts/drafts/IERC20Permit.sol");
                assert_eq!(res.0[32].contract_name, "contracts/interfaces/ISelfPermit.sol");
                assert_eq!(res.0[33].contract_name, "contracts/interfaces/external/IERC20PermitAllowed.sol");
                assert_eq!(res.0[34].contract_name, "contracts/libraries/BytesLib.sol");
            }
        }
    }
}
