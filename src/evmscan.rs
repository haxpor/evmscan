use crate::api::*;

/// Scale of decimals used to convert WEI back to native token in EVM-based chain.
///
/// FIXME: how to declare this while not using .pow() which is not constant function
pub static NATIVE_TOKEN_SCALE_F: f64 = 1_000_000_000_000_000_000_f64;

/// Get APIs in `Accounts` namespace.
/// NOTE: Users should cache the returned `Accounts` as this function will newly create
/// such instance every time when called although it's cheap.
pub fn accounts() -> accounts::Accounts {
    accounts::Accounts{}
}

/// Get APIs in `Stats` namespace.
/// NOTE: Users should cache the returned `Stats` as this function will newly create
/// such instance every time when called although it's cheap.
pub fn stats() -> stats::Stats {
    stats::Stats{}
}

/// Get Contracts in `Contracts` namespace.
/// NOTE: Users should cache the returned `Contracts` as this function will newly
/// create such instance every time called although it's cheap.
pub fn contracts() -> contracts::Contracts {
    contracts::Contracts{}
}
