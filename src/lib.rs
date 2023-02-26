use std::{cmp, ops};

pub trait Ishable {
    type Output;
    fn ish(&self) -> Self::Output;
}

impl Ishable for bool {
    type Output = BoolIsh;
    fn ish(&self) -> Self::Output {
        BoolIsh { value: *self }
    }
}

pub struct Ish;
#[allow(non_upper_case_globals)]
pub const ish: Ish = Ish;

impl ops::Sub<Ish> for bool {
    type Output = BoolIsh;

    fn sub(self, _rhs: Ish) -> Self::Output {
        BoolIsh { value: self }
    }
}

#[derive(Debug, Clone)]
pub struct BoolIsh {
    value: bool,
}

impl BoolIsh {
    fn is_true_string(&self, s: &str) -> bool {
        s.to_lowercase().trim().eq("true")
    }
}

impl cmp::PartialEq<&str> for BoolIsh {
    fn eq(&self, other: &&str) -> bool {
        if self.value {
            self.is_true_string(*other)
        } else {
            !self.is_true_string(*other)
        }
    }
}
impl cmp::PartialEq<String> for BoolIsh {
    fn eq(&self, other: &String) -> bool {
        if self.value {
            self.is_true_string(other.as_str())
        } else {
            !self.is_true_string(other.as_str())
        }
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

        assert!(trueish != "false");
        assert!(trueish != "False");
        assert!(trueish != "FALSE");
        assert!(trueish != "FALSE".to_owned());
        assert!(trueish != 0);
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
    }
}
