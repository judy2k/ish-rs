//! Ish — The fuzzy-equality library you never asked for.
//!
//! ```
//! use ish::ish;
//!
//! assert!(true-ish == "TRUE");
//! assert!(true-ish == "true");
//! assert!("True" == true-ish);
//! assert!(true-ish != "false");
//! assert!(true-ish != "penguins!");
//!
//! assert!(false-ish == "FALSE");
//! assert!(false-ish == "faLSE");
//! assert!(false-ish == "faLSE");
//! assert!(false-ish != "true");
//! assert!(false-ish != "ferret");
//! ```
//!
mod boolish;

pub use self::boolish::BoolIsh;

/// Ishable is a trait that can be implemented to indicate that a fuzzy-match type can be obtained by a value.
///
/// A type that implements Ishable has an `ish()` method that will return a fuzzy version of the object.
///
/// Currently the only provided implementation is on `bool`.
pub trait Ishable {
    type Output;
    fn ish(&self) -> Self::Output;
}

#[doc(hidden)]
pub struct Ish;

/// ish! The whole point of this library.
///
/// Subtract it from a bool and then compare the resulting object to integers and strings.
///
/// * `true-ish` is a vaguely truthy type.
/// * `false-ish` is a vaguely falsy type.
#[allow(non_upper_case_globals)]
pub const ish: Ish = Ish;
