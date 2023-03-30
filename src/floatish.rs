use crate::ISH_FUDGE_DEFAULT;
use crate::{Ish, Ishable};
use std::cmp;
use std::fmt::Debug;
use std::ops;

#[derive(Clone)]
pub struct FloatIsh {
    value: f64,
    fudge: f64,
}

impl Debug for FloatIsh {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ±{}", self.value, self.fudge)?;
        Ok(())
    }
}

impl Ishable for f64 {
    type Output = FloatIsh;
    fn ish(&self) -> Self::Output {
        FloatIsh {
            value: *self,
            fudge: ISH_FUDGE_DEFAULT,
        }
    }
}

impl ops::Sub<Ish> for f64 {
    type Output = FloatIsh;

    fn sub(self, rhs: Ish) -> Self::Output {
        FloatIsh {
            value: self,
            fudge: rhs.fudge,
        }
    }
}

impl ops::Sub<Ish> for f32 {
    type Output = FloatIsh;

    fn sub(self, rhs: Ish) -> Self::Output {
        FloatIsh {
            value: self as f64,
            fudge: rhs.fudge,
        }
    }
}

impl cmp::PartialEq<FloatIsh> for f64 {
    fn eq(&self, other: &FloatIsh) -> bool {
        (other.value - self).abs() <= other.fudge
    }
}

impl<T> cmp::PartialEq<T> for FloatIsh
where
    T: cmp::PartialEq<FloatIsh>,
{
    fn eq(&self, other: &T) -> bool {
        other.eq(self)
    }
}

#[cfg(test)]
mod test {
    use super::{Ish, Ishable};
    use crate::ish;

    #[test]
    fn test_f64() {
        assert_eq!(0.0 - ish, 0.0);
        assert_eq!(0.0 - ish, -0.000000000001);
        assert_eq!(0.0 - ish, 0.000000000001);
        assert_eq!(1.0 - ish, 1.0 + 0.000000000001);
        assert_eq!(1.0 - ish, 1.0 - 0.000000000001);

        assert_eq!(1.0 - 0.000000000001, 1.0 - ish);
        assert_eq!(-1.0 - ish, -1.0 - 0.000000000001);
        assert_eq!(-1.0 - ish, -1.0 + 0.000000000001);

        assert!(-1.0 - ish != -1.00001);
    }

    #[test]
    fn test_f32() {
        assert_eq!(0.0f32 - ish, -0.000000000001);
        assert_eq!(0.0f32 - ish, 0.000000000001);
        assert_eq!(1.0f32 - ish, 1.0 + 0.000000000001);
        assert_eq!(1.0f32 - ish, 1.0 - 0.000000000001);
        assert_eq!(-1.0f32 - ish, -1.0 - 0.000000000001);
        assert_eq!(-1.0f32 - ish, -1.0 + 0.000000000001);

        assert!(-1.0f32 - ish != -1.00001);
    }

    #[test]
    fn test_ish_method() {
        assert_eq!((0.0f64).ish(), -0.000000000001);
        assert_eq!((0.0).ish(), 0.000000000001);
        assert_eq!((1.0).ish(), 1.0 + 0.000000000001);
        assert_eq!((1.0).ish(), 1.0 - 0.000000000001);
        assert_eq!((-1.0).ish(), -1.0 - 0.000000000001);
        assert_eq!((-1.0).ish(), -1.0 + 0.000000000001);

        assert!((-1.0).ish() != -1.00001);
        assert!((-1.0).ish() != -2.0);
    }

    #[test]
    fn test_ish_function() {
        assert_eq!(-1.0 - Ish::new(0.001), -1.00001);
        assert!(-1.0 - Ish::new(0.001) != -1.1);
        assert!(-1.0 - Ish::new(-0.001) != -1.1);
    }

    #[test]
    fn test_floatish_debug() {
        assert_eq!(format!("{:?}", 3.1 - Ish::new(0.0001)), "3.1 ±0.0001");
    }
}
