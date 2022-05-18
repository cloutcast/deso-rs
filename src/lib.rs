//! Sign Transactions on the DeSo Blockchain. 
//! This crate is intended to sign transactions. 
//! 
//! This crate does NOT submit anything to DeSo at this time, and re-exports functionality from the base58check-encode crate.
//! 

pub use hex;
pub use base58check_encode as b58c;
pub mod seed_hex;

pub mod errors;
pub use errors::deso_signing_error as DesoSigningError;
pub use errors::from_mnemonic_error as FromMnemonicError;

#[cfg(test)]
mod tests;
