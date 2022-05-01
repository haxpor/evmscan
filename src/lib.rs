/// Common use types not directly related to EvmScan
pub mod types;

/// Deserializing implementation to support returned response back from APIs
pub mod deserialize;

/// Environment of working with this library
pub mod environ;

/// Main module that hold the core part of API implementations
pub mod evmscan;

/// Abstraction-level module to hold various group of APIs
pub mod api;
mod impls;

#[cfg(test)]
pub mod tests;

/// Most common types, and directly related types used in Bscscan
pub mod prelude {
    pub use primitive_types::*;
    pub use crate::types::*;
    pub use crate::types::Error as EvmError;
}
