#![warn(rust_2018_idioms)]

// The Rust Standard library gets to use these, but they should not be in this 3rd party software
// One day we can just use the stable niche syntax
// https://github.com/rust-lang/rfcs/pull/3334
#![feature(rustc_attrs)]

mod balanced;
pub use crate::balanced::BalancedI8;
pub use crate::balanced::BalancedI16;
pub use crate::balanced::BalancedI32;
pub use crate::balanced::BalancedI64;

