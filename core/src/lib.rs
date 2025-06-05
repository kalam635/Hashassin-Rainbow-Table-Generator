//! # Hashassin Core
//!
//! Library for password generation and hash computation
//!
//! ## Features
//! - Thread-safe password generation
//! - Multiple hashing algorithms
//! - Binary file format support

#![deny(clippy::unwrap_used, clippy::expect_used)]

pub mod dump;
pub mod hashing;
pub mod password;

pub use dump::dump_hashes;
pub use hashing::{Algorithm, HashError};
pub use password::PasswordError;

pub mod rainbow;
pub use rainbow::*;
