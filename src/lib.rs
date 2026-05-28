//! Purpose of this number to have "infinite" size and precision to calculate input and inverses,
//! for prop testing, fuzz setup, assertions and inverse calculations.
//! And double check engine logic regarding numerics and possible rounding limits/errors.
//! Usually such tests are written in Python, so we here go Rust approach.
//!
//! Math in engine is hard to read,
//! all these upcasts, downcasts, roundings, types, zero division checking, error handlings.
//! This number is slow, but works without all of that well.
//!
//! Maximal simplicity to cast from anything and integration with engine types is a way.
//!
//! Panics only on:
//! - division by zero (guess we should not use NaN as result to fail fast)
//! - failed conversion into smaller range types (instead of returning something which to be ? or unwrapped)
//!
//! So something like:
//! ```ignore
//! num!(42/7) - 33
//!
//! num!(44) / 3 - u128::MAX * num!(2/3).pow(3)
//! ```

use core::iter::Sum;
use core::ops::{Add, Div, Mul, Neg, Rem, Sub};

use malachite_base::num::arithmetic::traits::Pow as MalachitePow;
use malachite_q::Rational;

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Number(Rational);

impl Number {
    pub const fn new_i8(value: i8) -> Self {
        Self(Rational::const_from_signed(value as i64))
    }

    pub const fn new_i16(value: i16) -> Self {
        Self(Rational::const_from_signed(value as i64))
    }

    pub const fn new_i32(value: i32) -> Self {
        Self(Rational::const_from_signed(value as i64))
    }

    pub const fn new_i64(value: i64) -> Self {
        Self(Rational::const_from_signed(value))
    }

    pub const fn new_isize(value: isize) -> Self {
        Self(Rational::const_from_signed(value as i64))
    }

    pub fn new_i128(value: i128) -> Self {
        Self(Rational::from(value))
    }

    pub const fn new_nonzero_i8(value: core::num::NonZeroI8) -> Self {
        Self::new_i8(value.get())
    }

    pub const fn new_nonzero_i16(value: core::num::NonZeroI16) -> Self {
        Self::new_i16(value.get())
    }

    pub const fn new_nonzero_i32(value: core::num::NonZeroI32) -> Self {
        Self::new_i32(value.get())
    }

    pub const fn new_nonzero_i64(value: core::num::NonZeroI64) -> Self {
        Self::new_i64(value.get())
    }

    pub const fn new_nonzero_isize(value: core::num::NonZeroIsize) -> Self {
        Self::new_isize(value.get())
    }

    pub fn new_nonzero_i128(value: core::num::NonZeroI128) -> Self {
        Self::new_i128(value.get())
    }

    pub const fn new_u8(value: u8) -> Self {
        Self(Rational::const_from_unsigned(value as u64))
    }

    pub const fn new_u16(value: u16) -> Self {
        Self(Rational::const_from_unsigned(value as u64))
    }

    pub const fn new_u32(value: u32) -> Self {
        Self(Rational::const_from_unsigned(value as u64))
    }

    pub const fn new_u64(value: u64) -> Self {
        Self(Rational::const_from_unsigned(value))
    }

    pub const fn new_usize(value: usize) -> Self {
        Self(Rational::const_from_unsigned(value as u64))
    }

    pub fn new_u128(value: u128) -> Self {
        Self(Rational::from(value))
    }

    pub const fn new_nonzero_u8(value: core::num::NonZeroU8) -> Self {
        Self::new_u8(value.get())
    }

    pub const fn new_nonzero_u16(value: core::num::NonZeroU16) -> Self {
        Self::new_u16(value.get())
    }

    pub const fn new_nonzero_u32(value: core::num::NonZeroU32) -> Self {
        Self::new_u32(value.get())
    }

    pub const fn new_nonzero_u64(value: core::num::NonZeroU64) -> Self {
        Self::new_u64(value.get())
    }

    pub const fn new_nonzero_usize(value: core::num::NonZeroUsize) -> Self {
        Self::new_usize(value.get())
    }

    pub fn new_nonzero_u128(value: core::num::NonZeroU128) -> Self {
        Self::new_u128(value.get())
    }

    pub const fn new_ratio_i64(numerator: i64, denominator: i64) -> Self {
        Self(Rational::const_from_signeds(numerator, denominator))
    }

    pub fn new_ratio_i128(numerator: i128, denominator: i128) -> Self {
        Self(Rational::from_signeds(numerator, denominator))
    }

    #[doc(hidden)]
    pub const fn __parse_num_literal(value: &str) -> (u64, u64) {
        let bytes = value.as_bytes();
        let mut index = 0;
        let mut numerator = 0u64;
        let mut denominator = 1u64;
        let mut seen_decimal_point = false;
        let mut seen_digit = false;

        while index < bytes.len() {
            let byte = bytes[index];
            if byte == b'_' {
                index += 1;
                continue;
            }

            if byte == b'.' {
                if seen_decimal_point {
                    panic!("num literal should contain at most one decimal point");
                }
                seen_decimal_point = true;
                index += 1;
                continue;
            }

            if byte < b'0' || byte > b'9' {
                panic!("num literal should be an integer or finite decimal");
            }

            let digit = (byte - b'0') as u64;
            if numerator > (u64::MAX - digit) / 10 {
                panic!("num literal numerator should fit u64");
            }
            numerator = numerator * 10 + digit;
            if seen_decimal_point {
                if denominator > u64::MAX / 10 {
                    panic!("num literal denominator should fit u64");
                }
                denominator *= 10;
            }
            seen_digit = true;
            index += 1;
        }

        if !seen_digit {
            panic!("num literal should contain digits");
        }

        (numerator, denominator)
    }

    #[doc(hidden)]
    pub const fn __from_num_literal(value: &str, negative: bool) -> Self {
        let (numerator, denominator) = Self::__parse_num_literal(value);
        Self::__from_unsigned_ratio_parts(numerator, denominator, negative)
    }

    #[doc(hidden)]
    pub const fn __from_num_ratio_literals(
        numerator: &str,
        numerator_negative: bool,
        denominator: &str,
        denominator_negative: bool,
    ) -> Self {
        let (left_numerator, left_denominator) = Self::__parse_num_literal(numerator);
        let (right_numerator, right_denominator) = Self::__parse_num_literal(denominator);
        if right_numerator == 0 {
            panic!("num ratio denominator should not be zero");
        }

        let numerator = (left_numerator as u128) * (right_denominator as u128);
        let denominator = (left_denominator as u128) * (right_numerator as u128);
        if numerator > u64::MAX as u128 {
            panic!("num ratio numerator should fit u64");
        }
        if denominator > u64::MAX as u128 {
            panic!("num ratio denominator should fit u64");
        }

        Self::__from_unsigned_ratio_parts(
            numerator as u64,
            denominator as u64,
            numerator_negative != denominator_negative,
        )
    }

    pub(crate) const fn __from_unsigned_ratio_parts(
        numerator: u64,
        denominator: u64,
        negative: bool,
    ) -> Self {
        if denominator == 0 {
            panic!("num ratio denominator should not be zero");
        }

        if negative {
            if denominator > i64::MAX as u64 {
                panic!("negative num literal denominator should fit i64");
            }
            if numerator > i64::MAX as u64 + 1 {
                panic!("negative num literal numerator should fit i64");
            }
            let signed_numerator = if numerator == i64::MAX as u64 + 1 {
                i64::MIN
            } else {
                -(numerator as i64)
            };
            Self(Rational::const_from_signeds(
                signed_numerator,
                denominator as i64,
            ))
        } else {
            Self(Rational::const_from_unsigneds(numerator, denominator))
        }
    }

    pub fn pow(self, exponent: i16) -> Self {
        Self(MalachitePow::pow(self.0, i64::from(exponent)))
    }
}

#[macro_export]
macro_rules! num {
    (- $numerator:ident / - $denominator:tt) => {
        -$crate::Number::from($numerator) / -$crate::Number::from($denominator)
    };
    (- $numerator:ident / $denominator:tt) => {
        -$crate::Number::from($numerator) / $crate::Number::from($denominator)
    };
    (- $numerator:literal / - $denominator:literal) => {
        $crate::Number::__from_num_ratio_literals(
            stringify!($numerator),
            true,
            stringify!($denominator),
            true,
        )
    };
    (- $numerator:literal / $denominator:literal) => {
        $crate::Number::__from_num_ratio_literals(
            stringify!($numerator),
            true,
            stringify!($denominator),
            false,
        )
    };
    ($numerator:literal / - $denominator:literal) => {
        $crate::Number::__from_num_ratio_literals(
            stringify!($numerator),
            false,
            stringify!($denominator),
            true,
        )
    };
    ($numerator:literal / $denominator:literal) => {
        $crate::Number::__from_num_ratio_literals(
            stringify!($numerator),
            false,
            stringify!($denominator),
            false,
        )
    };
    (- $numerator:tt / - $denominator:tt) => {
        -$crate::Number::from($numerator) / -$crate::Number::from($denominator)
    };
    (- $numerator:tt / $denominator:tt) => {
        -$crate::Number::from($numerator) / $crate::Number::from($denominator)
    };
    ($numerator:tt / - $denominator:tt) => {
        $crate::Number::from($numerator) / -$crate::Number::from($denominator)
    };
    ($numerator:tt / $denominator:tt) => {
        $crate::Number::from($numerator) / $crate::Number::from($denominator)
    };
    (- $value:literal) => {
        $crate::Number::__from_num_literal(stringify!($value), true)
    };
    ($value:literal) => {
        $crate::Number::__from_num_literal(stringify!($value), false)
    };
    ($value:expr) => {
        $crate::Number::from($value)
    };
}

impl<T> Add<T> for Number
where
    T: Into<Number>,
{
    type Output = Number;

    fn add(self, rhs: T) -> Self::Output {
        Self(self.0 + rhs.into().0)
    }
}

impl<T> Add<T> for &Number
where
    T: Into<Number>,
{
    type Output = Number;

    fn add(self, rhs: T) -> Self::Output {
        Number(self.0.clone() + rhs.into().0)
    }
}

impl<T> Sub<T> for Number
where
    T: Into<Number>,
{
    type Output = Number;

    fn sub(self, rhs: T) -> Self::Output {
        Self(self.0 - rhs.into().0)
    }
}

impl<T> Sub<T> for &Number
where
    T: Into<Number>,
{
    type Output = Number;

    fn sub(self, rhs: T) -> Self::Output {
        Number(self.0.clone() - rhs.into().0)
    }
}

impl<T> Mul<T> for Number
where
    T: Into<Number>,
{
    type Output = Number;

    fn mul(self, rhs: T) -> Self::Output {
        Self(self.0 * rhs.into().0)
    }
}

impl<T> Mul<T> for &Number
where
    T: Into<Number>,
{
    type Output = Number;

    fn mul(self, rhs: T) -> Self::Output {
        Number(self.0.clone() * rhs.into().0)
    }
}

impl<T> Div<T> for Number
where
    T: Into<Number>,
{
    type Output = Number;

    fn div(self, rhs: T) -> Self::Output {
        Self(self.0 / rhs.into().0)
    }
}

impl<T> Div<T> for &Number
where
    T: Into<Number>,
{
    type Output = Number;

    fn div(self, rhs: T) -> Self::Output {
        Number(self.0.clone() / rhs.into().0)
    }
}

impl<T> Rem<T> for Number
where
    T: Into<Number>,
{
    type Output = Number;

    fn rem(self, rhs: T) -> Self::Output {
        Self(self.0 % rhs.into().0)
    }
}

impl<T> Rem<T> for &Number
where
    T: Into<Number>,
{
    type Output = Number;

    fn rem(self, rhs: T) -> Self::Output {
        Number(self.0.clone() % rhs.into().0)
    }
}

impl Neg for Number {
    type Output = Number;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl Neg for &Number {
    type Output = Number;

    fn neg(self) -> Self::Output {
        Number(-self.0.clone())
    }
}

impl Sum for Number {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::new_i64(0), Add::add)
    }
}

impl<'a> Sum<&'a Number> for Number {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Number>,
    {
        iter.fold(Self::new_i64(0), Add::add)
    }
}

macro_rules! impl_lhs_ops {
    ($type:ty) => {
        impl core::ops::Add<Number> for $type {
            type Output = Number;

            fn add(self, rhs: Number) -> Self::Output {
                Number::from(self) + rhs
            }
        }

        impl core::ops::Add<&Number> for $type {
            type Output = Number;

            fn add(self, rhs: &Number) -> Self::Output {
                Number::from(self) + rhs
            }
        }

        impl core::ops::Sub<Number> for $type {
            type Output = Number;

            fn sub(self, rhs: Number) -> Self::Output {
                Number::from(self) - rhs
            }
        }

        impl core::ops::Sub<&Number> for $type {
            type Output = Number;

            fn sub(self, rhs: &Number) -> Self::Output {
                Number::from(self) - rhs
            }
        }

        impl core::ops::Mul<Number> for $type {
            type Output = Number;

            fn mul(self, rhs: Number) -> Self::Output {
                Number::from(self) * rhs
            }
        }

        impl core::ops::Mul<&Number> for $type {
            type Output = Number;

            fn mul(self, rhs: &Number) -> Self::Output {
                Number::from(self) * rhs
            }
        }

        impl core::ops::Div<Number> for $type {
            type Output = Number;

            fn div(self, rhs: Number) -> Self::Output {
                Number::from(self) / rhs
            }
        }

        impl core::ops::Div<&Number> for $type {
            type Output = Number;

            fn div(self, rhs: &Number) -> Self::Output {
                Number::from(self) / rhs
            }
        }

        impl core::ops::Rem<Number> for $type {
            type Output = Number;

            fn rem(self, rhs: Number) -> Self::Output {
                Number::from(self) % rhs
            }
        }

        impl core::ops::Rem<&Number> for $type {
            type Output = Number;

            fn rem(self, rhs: &Number) -> Self::Output {
                Number::from(self) % rhs
            }
        }
    };
}

impl_lhs_ops!(i8);
impl_lhs_ops!(i16);
impl_lhs_ops!(i32);
impl_lhs_ops!(i64);
impl_lhs_ops!(i128);
impl_lhs_ops!(isize);
impl_lhs_ops!(u8);
impl_lhs_ops!(u16);
impl_lhs_ops!(u32);
impl_lhs_ops!(u64);
impl_lhs_ops!(u128);
impl_lhs_ops!(usize);
impl_lhs_ops!(core::num::NonZeroI8);
impl_lhs_ops!(core::num::NonZeroI16);
impl_lhs_ops!(core::num::NonZeroI32);
impl_lhs_ops!(core::num::NonZeroI64);
impl_lhs_ops!(core::num::NonZeroI128);
impl_lhs_ops!(core::num::NonZeroIsize);
impl_lhs_ops!(core::num::NonZeroU8);
impl_lhs_ops!(core::num::NonZeroU16);
impl_lhs_ops!(core::num::NonZeroU32);
impl_lhs_ops!(core::num::NonZeroU64);
impl_lhs_ops!(core::num::NonZeroU128);
impl_lhs_ops!(core::num::NonZeroUsize);
impl_lhs_ops!(bool);
impl_lhs_ops!(Option<bool>);
#[cfg(feature = "float")]
impl_lhs_ops!(f32);
#[cfg(feature = "float")]
impl_lhs_ops!(f64);

mod from;
mod string;
mod try_into;

pub use try_into::TryFromNumberError;

#[cfg(feature = "borsh")]
#[path = "encdec/borsh.rs"]
mod borsh;
#[cfg(feature = "float")]
#[path = "approximate/float.rs"]
mod float;
#[cfg(feature = "num-bigint")]
mod num_bigint;
#[cfg(feature = "num-rational")]
mod num_rational;
#[cfg(feature = "num-traits")]
mod num_traits;
#[cfg(any(feature = "borsh", feature = "scale", feature = "typical"))]
#[path = "encdec/rational_varint.rs"]
mod rational_varint;
#[cfg(feature = "ruint")]
mod ruint;
#[cfg(feature = "scale")]
#[path = "encdec/scale.rs"]
mod scale;
#[cfg(feature = "schemars")]
#[path = "encdec/schemars.rs"]
mod schemars;
#[cfg(feature = "serde")]
#[path = "encdec/serde.rs"]
mod serde;
#[cfg(feature = "typical")]
#[path = "encdec/typical.rs"]
pub mod typical;
