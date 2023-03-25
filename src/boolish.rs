use super::{Ish, Ishable};
use lazy_static::lazy_static;
use std::collections::HashSet;
use std::{cmp, ops};

lazy_static! {
    static ref TRUE_STRINGS: HashSet<String> = {
        let mut s = HashSet::new();
        s.insert("true".to_string());
        s.insert("yeah".to_string());
        s.insert("yup".to_string());
        s.insert("yes".to_string());
        s.insert("on".to_string());
        s.insert("👍".to_string());
        s.insert("yes way".to_string()); // Easter egg!
        s.insert("go for it".to_string());
        s
    };
    static ref FALSE_STRINGS: HashSet<String> = {
        let mut s = HashSet::new();
        s.insert("false".to_string());
        s.insert("untrue".to_string());
        s.insert("nope".to_string());
        s.insert("off".to_string());
        s.insert("nah".to_string());
        s.insert("naw".to_string());
        s.insert("nut".to_string());
        s.insert("👎".to_string());
        s.insert("no".to_string());
        s.insert("no way".to_string());
        s.insert("norway".to_string()); // Easter egg!
        s
    };
}

impl Ishable for bool {
    type Output = BoolIsh;
    fn ish(&self) -> Self::Output {
        BoolIsh { value: *self }
    }
}

impl ops::Sub<Ish> for bool {
    type Output = BoolIsh;

    fn sub(self, _rhs: Ish) -> Self::Output {
        BoolIsh { value: self }
    }
}

/// A fuzzy version of a boolean value.
///
/// Instances of BoolIsh can be obtained from either the expression
/// `true-ish` or `false-ish`.
/// These expressions will either return you a fuzzy true value,
/// or a fuzzy false value.
/// Variants of the word "true"
/// (such as "TRUE", "TrUe", "yes", "YUP", "on" and "👍")
/// are considered to be true-ish.
/// Also, the number 1 is `true-ish`.
/// Comparing anything that's not recognised to `true-ish` will return `false`.
///
/// Variants of the word "false"
/// (such as "FALSE" or "FaLse", "NO", "off", "Norway" and "naw")
/// are considered to be false-ish.
/// Also, the number 0 is `false-ish`.
/// Comparing anything that's not recognized to `false-ish` will return `false`.
#[derive(Debug, Clone)]
pub struct BoolIsh {
    value: bool,
}

impl<T, E> cmp::PartialEq<BoolIsh> for Result<T, E> {
    fn eq(&self, other: &BoolIsh) -> bool {
        if other.value {
            self.is_ok()
        } else {
            self.is_err()
        }
    }
}

impl<T> cmp::PartialEq<BoolIsh> for Option<T> {
    fn eq(&self, other: &BoolIsh) -> bool {
        if other.value {
            self.is_some()
        } else {
            self.is_none()
        }
    }
}

impl cmp::PartialEq<BoolIsh> for String {
    fn eq(&self, other: &BoolIsh) -> bool {
        string_fuzzy_eq(other.value, self)
    }
}

impl cmp::PartialEq<BoolIsh> for &str {
    fn eq(&self, other: &BoolIsh) -> bool {
        string_fuzzy_eq(other.value, self)
    }
}

fn string_fuzzy_eq(value: bool, other: &str) -> bool {
    if value {
        TRUE_STRINGS.contains(&other.to_lowercase())
    } else {
        FALSE_STRINGS.contains(&other.to_lowercase())
    }
}

fn i64_fuzzy_eq(value: bool, other: i64) -> bool {
    if value {
        other == 1
    } else {
        other != 1
    }
}

impl cmp::PartialEq<BoolIsh> for i8 {
    fn eq(&self, other: &BoolIsh) -> bool {
        i64_fuzzy_eq(other.value, *self as i64)
    }
}

impl cmp::PartialEq<BoolIsh> for i16 {
    fn eq(&self, other: &BoolIsh) -> bool {
        i64_fuzzy_eq(other.value, *self as i64)
    }
}

impl cmp::PartialEq<BoolIsh> for i32 {
    fn eq(&self, other: &BoolIsh) -> bool {
        i64_fuzzy_eq(other.value, *self as i64)
    }
}

impl cmp::PartialEq<BoolIsh> for i64 {
    fn eq(&self, other: &BoolIsh) -> bool {
        i64_fuzzy_eq(other.value, *self)
    }
}

impl cmp::PartialEq<BoolIsh> for isize {
    fn eq(&self, other: &BoolIsh) -> bool {
        i64_fuzzy_eq(other.value, *self as i64)
    }
}

impl cmp::PartialEq<BoolIsh> for u8 {
    fn eq(&self, other: &BoolIsh) -> bool {
        i64_fuzzy_eq(other.value, *self as i64)
    }
}

impl cmp::PartialEq<BoolIsh> for u16 {
    fn eq(&self, other: &BoolIsh) -> bool {
        i64_fuzzy_eq(other.value, *self as i64)
    }
}

impl cmp::PartialEq<BoolIsh> for u32 {
    fn eq(&self, other: &BoolIsh) -> bool {
        i64_fuzzy_eq(other.value, *self as i64)
    }
}

impl cmp::PartialEq<BoolIsh> for u64 {
    fn eq(&self, other: &BoolIsh) -> bool {
        i64_fuzzy_eq(other.value, *self as i64)
    }
}

impl cmp::PartialEq<BoolIsh> for usize {
    fn eq(&self, other: &BoolIsh) -> bool {
        i64_fuzzy_eq(other.value, *self as i64)
    }
}

impl<T> cmp::PartialEq<T> for BoolIsh
where
    T: cmp::PartialEq<BoolIsh>,
{
    fn eq(&self, other: &T) -> bool {
        other.eq(self)
    }
}

#[cfg(test)]
mod tests {
    use super::super::ish;
    use super::*;

    #[test]
    fn test_true_minus_ish() {
        let trueish = true - ish;

        true_assertions(trueish);
    }

    #[test]
    fn test_true_dot_ish() {
        let trueish = true.ish();

        true_assertions(trueish);
    }

    fn true_assertions(trueish: BoolIsh) {
        assert_eq!(trueish, "true");
        assert_eq!(trueish, "True");
        assert_eq!(trueish, "TRUE");
        assert_eq!(trueish, "TRUE".to_owned());
        assert_eq!(trueish, "👍");

        assert!(trueish == Ok::<(), ()>(()));
        assert!(trueish == Some(()));

        assert_eq!(trueish, 1);
        assert_eq!(trueish, 1i8);
        assert_eq!(trueish, 1i16);
        assert_eq!(trueish, 1i32);
        assert_eq!(trueish, 1isize);
        assert_eq!(trueish, 1u8);
        assert_eq!(trueish, 1u16);
        assert_eq!(trueish, 1u32);
        assert_eq!(trueish, 1u64);
        assert_eq!(trueish, 1usize);

        assert!(trueish != "penguin");
        assert!(trueish != "false");
        assert!(trueish != "False");
        assert!(trueish != "FALSE");
        assert!(trueish != "FALSE".to_owned());
        assert!(trueish != 0);
        assert!(trueish != 0i32);
        assert!(trueish != 0i64);
        assert!(trueish != 0isize);
        assert!(trueish != 0usize);

        assert!(trueish != Err::<(), ()>(()));
        assert!(trueish != None::<()>);

        assert!("True" == trueish);
        assert!(1 == trueish);
    }

    #[test]
    fn test_falseish() {
        let falseish = false - ish;

        assert_eq!(falseish, "false");
        assert_eq!(falseish, "False");
        assert_eq!(falseish, "FALSE");
        assert_eq!(falseish, "FALSE".to_owned());
        assert_eq!(falseish, 0);
        assert_eq!(falseish, "👎");
        assert_eq!(falseish, Err::<(), ()>(()));
        assert_eq!(falseish, None::<()>);

        assert!(falseish != "true");
        assert!(falseish != "True");
        assert!(falseish != "TRUE");
        assert!(falseish != "TRUE".to_owned());
        assert!(falseish != 1);
        assert!(falseish != "ferret");
        assert!(falseish != Ok::<(), ()>(()));
        assert!(falseish != Some(()));

        assert!("False" == falseish);
        assert!(0 == falseish);
    }
}
