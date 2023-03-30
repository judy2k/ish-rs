use crate::{Ish, Ishable};
use std::cmp;
use std::ops;

#[derive(Debug, Clone)]
pub struct FloatIsh {
    value: f64,
}

impl Ishable for f64 {
    type Output = FloatIsh;
    fn ish(&self) -> Self::Output {
        FloatIsh { value: *self }
    }
}

impl ops::Sub<Ish> for f64 {
    type Output = FloatIsh;

    fn sub(self, _rhs: Ish) -> Self::Output {
        FloatIsh { value: self }
    }
}

impl ops::Sub<Ish> for f32 {
    type Output = FloatIsh;

    fn sub(self, _rhs: Ish) -> Self::Output {
        FloatIsh { value: self as f64 }
    }
}

impl cmp::PartialEq<FloatIsh> for f64 {
    fn eq(&self, other: &FloatIsh) -> bool {
        other.value.abs() - (self).abs() <= 0.00000000001
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
    use super::Ishable;
    use crate::ish;

    #[test]
    fn test_f64() {
        assert_eq!(0.0 - ish, -0.001);
        assert_eq!(0.0 - ish, 0.001);
        assert_eq!(1.0 - ish, 1.001);
        assert_eq!(1.0 - ish, 1.0 - 0.000000000001);

        assert_eq!(1.0 - 0.000000000001, 1.0 - ish);
        assert_eq!(-1.0 - ish, -1.001);
        assert_eq!(-1.0 - ish, 0.000000000001 - 1.0);
    }

    #[test]
    fn test_f32() {
        assert_eq!(0.0f32 - ish, -0.001);
        assert_eq!(0.0f32 - ish, 0.001);
        assert_eq!(1.0f32 - ish, 1.001);
        assert_eq!(1.0f32 - ish, 1.0 - 0.000000000001);
        assert_eq!(-1.0f32 - ish, -1.001);
        assert_eq!(-1.0f32 - ish,  0.000000000001 - 1.0);
    }

    #[test]
    fn test_ish_method() {
        assert_eq!((0.0f64).ish(), -0.001);
        assert_eq!((0.0).ish(), 0.001);
        assert_eq!((1.0).ish(), 1.001);
        assert_eq!((1.0).ish(),  1.0 - 0.000000000001);
        assert_eq!((-1.0).ish(), -1.001);
        assert_eq!((-1.0).ish(), 0.000000000001 - 1.0);
    }
}
