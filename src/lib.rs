//! # Grepme
//!
//! A library that provides utilities necessary for grep-like operations.
//!
//! The crate is organised into a few focused modules:
//!
//! - [`config`] — parsing of command-line arguments into a [`Config`], plus
//!   the CLI help text.
//! - [`targets`] — expanding file/directory arguments into a flat list of
//!   files to search.
//! - [`search`] — running a compiled regex against file contents.
pub mod config;
pub mod search;
pub mod targets;

pub use targets::ExpansionFailed;
