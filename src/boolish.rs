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
        s
    };
    static ref FALSE_STRINGS: HashSet<String> = {
        let mut s = HashSet::new();
        s.insert("false".to_string());
        s.insert("untrue".to_string());
        s.insert("nope".to_string());
        s.insert("no".to_string());
        s.insert("off".to_string());
        s.insert("nah".to_string());
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
/// (such as "TRUE" or "TrUe")
/// are considered to be true-ish.
/// Comparing anything else to `true-ish` will return `false`.
/// Also, the number 1 is `true-ish`.
///
/// Variants of the word "false"
/// (such as "FALSE" or "FaLse")
/// are considered to be false-ish.
/// Comparing anything else to `false-ish` will return `false`.
/// Also, the number 0 is `false-ish`.
#[derive(Debug, Clone, PartialEq)]
pub struct BoolIsh {
    value: bool,
}

impl BoolIsh {
    fn is_true_string(&self, s: &str) -> bool {
        TRUE_STRINGS.contains(&s.to_lowercase())
    }

    fn is_false_string(&self, s: &str) -> bool {
        FALSE_STRINGS.contains(&s.to_lowercase())
    }
}

impl cmp::PartialEq<&str> for BoolIsh {
    fn eq(&self, other: &&str) -> bool {
        if self.value {
            self.is_true_string(*other)
        } else {
            self.is_false_string(*other)
        }
    }
}
impl cmp::PartialEq<String> for BoolIsh {
    fn eq(&self, other: &String) -> bool {
        if self.value {
            self.is_true_string(other.as_str())
        } else {
            self.is_false_string(other.as_str())
        }
    }
}

impl cmp::PartialEq<BoolIsh> for String {
    fn eq(&self, other: &BoolIsh) -> bool {
        other.eq(self)
    }
}

impl cmp::PartialEq<BoolIsh> for &str {
    fn eq(&self, other: &BoolIsh) -> bool {
        other.eq(self)
    }
}

fn i64_fuzzy_eq(value: bool, other: i64) -> bool {
    if value {
        other == 1
    } else {
        other != 1
    }
}

impl cmp::PartialEq<u8> for BoolIsh {
    fn eq(&self, other: &u8) -> bool {
        i64_fuzzy_eq(self.value, *other as i64)
    }
}
impl cmp::PartialEq<u16> for BoolIsh {
    fn eq(&self, other: &u16) -> bool {
        i64_fuzzy_eq(self.value, *other as i64)
    }
}
impl cmp::PartialEq<u32> for BoolIsh {
    fn eq(&self, other: &u32) -> bool {
        i64_fuzzy_eq(self.value, *other as i64)
    }
}
impl cmp::PartialEq<u64> for BoolIsh {
    fn eq(&self, other: &u64) -> bool {
        i64_fuzzy_eq(self.value, *other as i64)
    }
}
impl cmp::PartialEq<usize> for BoolIsh {
    fn eq(&self, other: &usize) -> bool {
        i64_fuzzy_eq(self.value, *other as i64)
    }
}
impl cmp::PartialEq<i8> for BoolIsh {
    fn eq(&self, other: &i8) -> bool {
        i64_fuzzy_eq(self.value, *other as i64)
    }
}
impl cmp::PartialEq<i16> for BoolIsh {
    fn eq(&self, other: &i16) -> bool {
        i64_fuzzy_eq(self.value, *other as i64)
    }
}
impl cmp::PartialEq<i32> for BoolIsh {
    fn eq(&self, other: &i32) -> bool {
        i64_fuzzy_eq(self.value, *other as i64)
    }
}
impl cmp::PartialEq<i64> for BoolIsh {
    fn eq(&self, other: &i64) -> bool {
        i64_fuzzy_eq(self.value, *other as i64)
    }
}
impl cmp::PartialEq<isize> for BoolIsh {
    fn eq(&self, other: &isize) -> bool {
        i64_fuzzy_eq(self.value, *other as i64)
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
        assert_eq!(trueish, 1);
        assert_eq!(trueish, 1i32);
        assert_eq!(trueish, 1i64);
        assert_eq!(trueish, 1isize);
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
    }

    #[test]
    fn test_falseish() {
        let falseish = false - ish;

        assert_eq!(falseish, "false");
        assert_eq!(falseish, "False");
        assert_eq!(falseish, "FALSE");
        assert_eq!(falseish, "FALSE".to_owned());
        assert_eq!(falseish, 0);

        assert!(falseish != "true");
        assert!(falseish != "True");
        assert!(falseish != "TRUE");
        assert!(falseish != "TRUE".to_owned());
        assert!(falseish != 1);
        assert!(falseish != "ferret");
    }
}
