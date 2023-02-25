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

    fn is_true_u64(&self, s: u64) -> bool {
        s == 1
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

impl cmp::PartialEq<u8> for BoolIsh {
    fn eq(&self, other: &u8) -> bool {
        if self.value {
            self.is_true_u64(*other as u64)
        } else {
            !self.is_true_u64(*other as u64)
        }
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
        assert_eq!(trueish, 1);

        assert!(trueish != "false");
        assert!(trueish != "False");
        assert!(trueish != "FALSE");
        assert!(trueish != 0);
    }

    #[test]
    fn test_falseish() {
        let falseish = false - ish;

        assert_eq!(falseish, "false");
        assert_eq!(falseish, "False");
        assert_eq!(falseish, "FALSE");
        assert_eq!(falseish, 0);

        assert!(falseish != "true");
        assert!(falseish != "True");
        assert!(falseish != "TRUE");
        assert!(falseish != 1);
    }
}
