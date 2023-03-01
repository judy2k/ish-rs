//! Ish — The fuzzy-equality library you never asked for.
//!
//! Sometimes things aren't quite true or false,
//! they're more like true-ish or false-ish.
//!
//! ```
//! use ish::ish;
//!
//! // Some examples of true-ish values:
//! assert!(true-ish == "TRUE");
//! assert!(true-ish == "true");
//! assert!(true-ish == "on");
//! assert!(true-ish == "YEAH");
//! assert!(true-ish == "👍");
//! assert!(true-ish == 1);
//! assert!("True" == true-ish); // Comparison works in both directions.
//!
//! // The following values are not true-ish:
//! assert!(true-ish != 0);
//! assert!(true-ish != "false");
//! assert!(true-ish != "penguins!");
//!
//! // Some examples of false-ish values:
//! assert!(false-ish == "FALSE");
//! assert!(false-ish == "off");
//! assert!(false-ish == "nope");
//! assert!(false-ish == "no");
//! assert!(false-ish == "Norway"); // Easter egg!
//! assert!(false-ish == "faLSE");
//! assert!(false-ish == "👎");
//! assert!(false-ish == 0);
//!
//! // The following values are *not* false-ish:
//! assert!(false-ish != "nopeee");
//! assert!(false-ish != 1);
//! assert!(false-ish != "true");
//! assert!(false-ish != "ferret");
//! ```
//!
//! Note that ish tries to be a *little* conservative about deciding if
//! something is `true-ish` or `false-ish`,
//! so if a value is unrecognised as either thruth-y or false-y, then it will
//! not match as equal to either `truth-ish` or `false-ish`.
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
