//! Purpose of this number to have "infinite" size and precision to calculate input and inverses,
//! for prop testing, fuzz setup, assertions and inverse calculations.
//! And double check engine logic regarding numerics and possible rouning limits/errors.
//! Usually such tests are written in Python, so we here go Rust approach.
//!
//! Math in engine is hard to read,
//! all these upcasts, downcasts, roundings, types, zero division checking, error handlings.
//! This number is slow, but works without all of that well.
//!
//! Maximal simplicity to cast from anything and itegration with engine types is a way.
//!
//! Panics only on:
//! - division by zero (guess we should not use NaN as result to fail fast)
//! - failed conversion into smaller range types (instead of returing something which to be ? or unwrapped)
//!
//! So something like:
//! ```ignore
//! num!(42/7) - 33
//!
//! num!(44) / 3 - u128::MAX * num!(2/3).pow(3)
//! ```

use core::fmt;
use core::iter::Sum;
use core::ops::{Add, Div, Mul, Neg, Rem, Sub};

use malachite_base::num::arithmetic::traits::Pow as MalachitePow;
use malachite_nz::natural::Natural;
use malachite_q::Rational;

const DEBUG_FRACTIONAL_DIGITS: usize = 32;

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

    const fn __from_unsigned_ratio_parts(numerator: u64, denominator: u64, negative: bool) -> Self {
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

    #[cfg(feature = "num-bigint")]
    pub fn new_num_bigint(value: num_bigint::BigInt) -> Self {
        Self(parse_rational(&value.to_string()))
    }

    #[cfg(feature = "num-bigint")]
    pub fn new_num_biguint(value: num_bigint::BigUint) -> Self {
        Self(parse_rational(&value.to_string()))
    }

    #[cfg(feature = "num-ration")]
    pub fn new_num_rational<T>(value: num_rational::Ratio<T>) -> Self
    where
        T: core::fmt::Display,
    {
        let (numerator, denominator) = value.into_raw();
        Self(parse_rational(&format!("{numerator}/{denominator}")))
    }

    #[cfg(feature = "ruint")]
    pub fn new_ruint<const BITS: usize, const LIMBS: usize>(
        value: ruint::Uint<BITS, LIMBS>,
    ) -> Self {
        Self(parse_rational(&value.to_string()))
    }
}

impl fmt::Debug for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rational = self.0.to_string();
        if !rational.contains('/') {
            return f.write_str(&rational);
        }

        if rational.starts_with('-') {
            f.write_str("-")?;
        }

        let (mut before_point, after_point) = self.0.to_digits(&Natural::from(10u32));
        if before_point.is_empty() {
            f.write_str("0")?;
        } else {
            while let Some(digit) = before_point.pop() {
                f.write_str(&digit.to_string())?;
            }
        }

        f.write_str(".")?;
        let digits_to_write = after_point.len().unwrap_or(DEBUG_FRACTIONAL_DIGITS);
        for index in 0..digits_to_write {
            let digit = after_point
                .get(index)
                .expect("fractional digit should exist");
            f.write_str(&digit.to_string())?;
        }
        if !after_point.is_finite() {
            f.write_str("...")?;
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! num {
    (- $value:literal) => {
        $crate::Number::__from_num_literal(stringify!($value), true)
    };
    ($value:literal) => {
        $crate::Number::__from_num_literal(stringify!($value), false)
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
}

#[cfg(any(
    feature = "borsh",
    feature = "num-bigint",
    feature = "num-ration",
    feature = "ruint",
    feature = "serde"
))]
fn parse_rational(value: &str) -> Rational {
    value
        .parse()
        .expect("num value should parse as malachite rational")
}

#[cfg(feature = "serde")]
impl serde::Serialize for Number {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Number {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <&str>::deserialize(deserializer)?;
        value
            .parse()
            .map(Self)
            .map_err(|()| serde::de::Error::custom("invalid rational"))
    }
}

#[cfg(feature = "borsh")]
impl borsh::BorshSerialize for Number {
    fn serialize<W: borsh::io::Write>(&self, writer: &mut W) -> borsh::io::Result<()> {
        borsh::BorshSerialize::serialize(self.0.to_string().as_bytes(), writer)
    }
}

#[cfg(feature = "borsh")]
impl borsh::BorshDeserialize for Number {
    fn deserialize_reader<R: borsh::io::Read>(reader: &mut R) -> borsh::io::Result<Self> {
        let bytes = <Vec<u8> as borsh::BorshDeserialize>::deserialize_reader(reader)?;
        let value = core::str::from_utf8(&bytes)
            .map_err(|error| borsh::io::Error::new(borsh::io::ErrorKind::InvalidData, error))?;
        value.parse().map(Self).map_err(|()| {
            borsh::io::Error::new(borsh::io::ErrorKind::InvalidData, "invalid rational")
        })
    }
}

#[cfg(feature = "num-traits")]
impl num_traits::Zero for Number {
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

#[cfg(feature = "num-traits")]
impl num_traits::One for Number {
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

#[cfg(feature = "num-traits")]
impl num_traits::Num for Number {
    type FromStrRadixErr = ();

    fn from_str_radix(value: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        if radix != 10 {
            return Err(());
        }
        value.parse().map(Self)
    }
}

#[cfg(feature = "num-traits")]
impl num_traits::Signed for Number {
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

#[cfg(feature = "num-traits")]
impl num_traits::Inv for Number {
    type Output = Number;

    fn inv(self) -> Self::Output {
        Self::new_i64(1) / self
    }
}

#[cfg(feature = "num-traits")]
impl num_traits::Inv for &Number {
    type Output = Number;

    fn inv(self) -> Self::Output {
        Number::new_i64(1) / self
    }
}

#[cfg(feature = "num-traits")]
impl num_traits::Pow<i16> for Number {
    type Output = Number;

    fn pow(self, exponent: i16) -> Self::Output {
        Number::pow(self, exponent)
    }
}

#[cfg(feature = "num-traits")]
impl num_traits::Pow<i16> for &Number {
    type Output = Number;

    fn pow(self, exponent: i16) -> Self::Output {
        Number::pow(self.clone(), exponent)
    }
}

macro_rules! impl_from_nonzero {
    ($type:ty, $constructor:ident) => {
        impl From<$type> for Number {
            fn from(value: $type) -> Self {
                Self::$constructor(value)
            }
        }
    };
}

macro_rules! impl_from_primitive {
    ($type:ty, $constructor:ident) => {
        impl From<$type> for Number {
            fn from(value: $type) -> Self {
                Self::$constructor(value)
            }
        }
    };
}

impl From<&Number> for Number {
    fn from(value: &Number) -> Self {
        value.clone()
    }
}

impl_from_primitive!(i8, new_i8);
impl_from_primitive!(i16, new_i16);
impl_from_primitive!(i32, new_i32);
impl_from_primitive!(i64, new_i64);
impl_from_primitive!(i128, new_i128);
impl_from_primitive!(isize, new_isize);
impl_from_primitive!(u8, new_u8);
impl_from_primitive!(u16, new_u16);
impl_from_primitive!(u32, new_u32);
impl_from_primitive!(u64, new_u64);
impl_from_primitive!(u128, new_u128);
impl_from_primitive!(usize, new_usize);

impl_from_nonzero!(core::num::NonZeroI8, new_nonzero_i8);
impl_from_nonzero!(core::num::NonZeroI16, new_nonzero_i16);
impl_from_nonzero!(core::num::NonZeroI32, new_nonzero_i32);
impl_from_nonzero!(core::num::NonZeroI64, new_nonzero_i64);
impl_from_nonzero!(core::num::NonZeroI128, new_nonzero_i128);
impl_from_nonzero!(core::num::NonZeroIsize, new_nonzero_isize);
impl_from_nonzero!(core::num::NonZeroU8, new_nonzero_u8);
impl_from_nonzero!(core::num::NonZeroU16, new_nonzero_u16);
impl_from_nonzero!(core::num::NonZeroU32, new_nonzero_u32);
impl_from_nonzero!(core::num::NonZeroU64, new_nonzero_u64);
impl_from_nonzero!(core::num::NonZeroU128, new_nonzero_u128);
impl_from_nonzero!(core::num::NonZeroUsize, new_nonzero_usize);

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
        impl Add<Number> for $type {
            type Output = Number;

            fn add(self, rhs: Number) -> Self::Output {
                Number::from(self) + rhs
            }
        }

        impl Add<&Number> for $type {
            type Output = Number;

            fn add(self, rhs: &Number) -> Self::Output {
                Number::from(self) + rhs
            }
        }

        impl Sub<Number> for $type {
            type Output = Number;

            fn sub(self, rhs: Number) -> Self::Output {
                Number::from(self) - rhs
            }
        }

        impl Sub<&Number> for $type {
            type Output = Number;

            fn sub(self, rhs: &Number) -> Self::Output {
                Number::from(self) - rhs
            }
        }

        impl Mul<Number> for $type {
            type Output = Number;

            fn mul(self, rhs: Number) -> Self::Output {
                Number::from(self) * rhs
            }
        }

        impl Mul<&Number> for $type {
            type Output = Number;

            fn mul(self, rhs: &Number) -> Self::Output {
                Number::from(self) * rhs
            }
        }

        impl Div<Number> for $type {
            type Output = Number;

            fn div(self, rhs: Number) -> Self::Output {
                Number::from(self) / rhs
            }
        }

        impl Div<&Number> for $type {
            type Output = Number;

            fn div(self, rhs: &Number) -> Self::Output {
                Number::from(self) / rhs
            }
        }

        impl Rem<Number> for $type {
            type Output = Number;

            fn rem(self, rhs: Number) -> Self::Output {
                Number::from(self) % rhs
            }
        }

        impl Rem<&Number> for $type {
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

#[cfg(feature = "num-bigint")]
impl_lhs_ops!(num_bigint::BigInt);
#[cfg(feature = "num-bigint")]
impl_lhs_ops!(num_bigint::BigUint);

#[cfg(feature = "num-bigint")]
impl From<num_bigint::BigInt> for Number {
    fn from(value: num_bigint::BigInt) -> Self {
        Self::new_num_bigint(value)
    }
}

#[cfg(feature = "num-bigint")]
impl From<num_bigint::BigUint> for Number {
    fn from(value: num_bigint::BigUint) -> Self {
        Self::new_num_biguint(value)
    }
}

#[cfg(feature = "num-ration")]
impl<T> From<num_rational::Ratio<T>> for Number
where
    T: core::fmt::Display,
{
    fn from(value: num_rational::Ratio<T>) -> Self {
        Self::new_num_rational(value)
    }
}

#[cfg(feature = "ruint")]
impl<const BITS: usize, const LIMBS: usize> From<ruint::Uint<BITS, LIMBS>> for Number {
    fn from(value: ruint::Uint<BITS, LIMBS>) -> Self {
        Self::new_ruint(value)
    }
}
