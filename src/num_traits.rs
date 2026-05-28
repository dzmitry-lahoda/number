use crate::Number;

impl ::num_traits::Zero for Number {
    fn zero() -> Self {
        Self::new_i64(0)
    }

    fn set_zero(&mut self) {
        *self = Self::zero();
    }

    fn is_zero(&self) -> bool {
        self == &Self::zero()
    }
}

impl ::num_traits::One for Number {
    fn one() -> Self {
        Self::new_i64(1)
    }

    fn set_one(&mut self) {
        *self = Self::one();
    }

    fn is_one(&self) -> bool {
        self == &Self::one()
    }
}

impl ::num_traits::Num for Number {
    type FromStrRadixErr = ();

    fn from_str_radix(value: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        if radix != 10 {
            return Err(());
        }
        value.parse()
    }
}

impl ::num_traits::Signed for Number {
    fn abs(&self) -> Self {
        if self.is_negative() {
            -self
        } else {
            self.clone()
        }
    }

    fn abs_sub(&self, other: &Self) -> Self {
        if self <= other {
            Self::new_i64(0)
        } else {
            self - other
        }
    }

    fn signum(&self) -> Self {
        if self.is_positive() {
            Self::new_i64(1)
        } else if self.is_negative() {
            Self::new_i64(-1)
        } else {
            Self::new_i64(0)
        }
    }

    fn is_positive(&self) -> bool {
        self > &Self::new_i64(0)
    }

    fn is_negative(&self) -> bool {
        self < &Self::new_i64(0)
    }
}

impl ::num_traits::Inv for Number {
    type Output = Number;

    fn inv(self) -> Self::Output {
        Self::new_i64(1) / self
    }
}

impl ::num_traits::Inv for &Number {
    type Output = Number;

    fn inv(self) -> Self::Output {
        Number::new_i64(1) / self
    }
}

macro_rules! impl_pow {
    ($exponent:ty) => {
        impl ::num_traits::Pow<$exponent> for Number {
            type Output = Number;

            fn pow(self, exponent: $exponent) -> Self::Output {
                Number::pow(self, i16::from(exponent))
            }
        }

        impl ::num_traits::Pow<$exponent> for &Number {
            type Output = Number;

            fn pow(self, exponent: $exponent) -> Self::Output {
                Number::pow(self.clone(), i16::from(exponent))
            }
        }
    };
}

impl_pow!(i8);
impl_pow!(i16);
impl_pow!(u8);
