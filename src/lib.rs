//! Rust utilities with various implementations of classes which cannot
//! be found in the rust standard library
//!
//! # Usage
//!
//! This repository is not available as crate. Classes of interest must
//! be directly added to the project

// enable additional rustc warnings
#![warn(
    anonymous_parameters,
    missing_debug_implementations,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unstable_features,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    variant_size_differences
)]
// enable additional clippy warnings
#![cfg_attr(feature = "cargo-clippy", warn(int_plus_one))]
#![cfg_attr(
    feature = "cargo-clippy",
    warn(shadow_reuse, shadow_same, shadow_unrelated)
)]
#![cfg_attr(feature = "cargo-clippy", warn(mut_mut))]
#![cfg_attr(feature = "cargo-clippy", warn(nonminimal_bool))]
#![cfg_attr(feature = "cargo-clippy", warn(pub_enum_variant_names))]
#![cfg_attr(feature = "cargo-clippy", warn(range_plus_one))]
#![cfg_attr(
    feature = "cargo-clippy",
    warn(string_add, string_add_assign)
)]
#![cfg_attr(feature = "cargo-clippy", warn(stutter))]
#![cfg_attr(feature = "cargo-clippy", warn(result_unwrap_used))]

#[cfg(test)]
extern crate rand;

pub mod btrie;
pub mod xor_linked_list;

#[doc(inline)]
pub use self::btrie::BTrieMap;
#[doc(inline)]
pub use self::xor_linked_list::XorLinkedList;
