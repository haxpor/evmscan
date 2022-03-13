use crate::api::*;

/// Scale of decimals used to convert WEI back to BNB
/// See unit of conversion at
/// [https://bscscan.com/unitconverter](https://bscscan.com/unitconverter)
///
/// FIXME: how to declare this while not using .pow() which is not constant function
pub static BNB_SCALE_F: f64 = 1_000_000_000_000_000_000_f64;

/// Get APIs in `Accounts` namespace
/// Users should cache the returned `Accounts` as this function will newly create
/// such instance every time when called.
pub fn accounts() -> accounts::Accounts {
    accounts::Accounts{}
}
