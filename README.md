# Nook

Nook will become a collection of Rust types with niches

## Niches

Rust types can have a "niche" which will be used during type layout to make containing types smaller than they'd otherwise need to be.
For example`&T` is the same size as `Option<&T>`.

The Rust standard library includes `std::num::NonZeroIsize` and similar non-zero types with this property. Today we cannot add niches to our own types without nightly Rust features,
and so that's what this crate does. But since the existence of niches is crucial to Rust's design, some day they'll be stabilized:

[joshtriplett's Proposed RFC](https://github.com/rust-lang/rfcs/pull/3334) is the current effort towards stabilisation.

The intent is that Nook will grow to include other types which have two desirable properties: They make sense as types, and yet they also offer a niche


## Types so far

`nook::BalancedI8` `nook::BalancedI16` `nook::BalancedI32` `nook::BalancedI64` 

Balanced integers are a pleasant and in some ways easier to use integral type lacking the most negative value from ordinary signed integers

