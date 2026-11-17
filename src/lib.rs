//! # TinyCrypt Safe Rust
//!
//! 100% Safe Rust (`#![forbid(unsafe_code)]`) and `no_std` cryptographic library,
//! deterministically lifted from Intel's TinyCrypt library by the DecodeIQ CodeTruth Engine.
//!
//! Zero raw pointers, zero heap allocation, guaranteed memory safety for embedded & bare-metal systems.

#![no_std]
#![forbid(unsafe_code)]

pub mod utils;
pub mod sha256;
pub mod aes;

pub use utils::constant_time_compare;
pub use sha256::Sha256;
pub use aes::AES_ctx;
