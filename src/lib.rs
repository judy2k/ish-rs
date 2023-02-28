mod boolish;

pub use crate::boolish::BoolIsh;

pub trait Ishable {
    type Output;
    fn ish(&self) -> Self::Output;
}

pub struct Ish;
#[allow(non_upper_case_globals)]
pub const ish: Ish = Ish;

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
