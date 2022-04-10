//! Schema types and parsers for defining the shape and semantics of FFXIV Excel
//! data structures.

// Lint config
#![allow(clippy::module_inception)]
#![warn(missing_debug_implementations, missing_docs)]

mod schema;

#[cfg(feature = "saint_coinach")]
pub mod saint_coinach;

pub use schema::{Node, Order, Sheet};