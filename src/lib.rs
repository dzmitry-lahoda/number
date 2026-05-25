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
//! num!(42.2) - 33i8
//!
//! num!(44) / 3i32 - u128::MAX * num!(2/3).pow(3i8)
//! ```

use core::fmt;

use malachite_nz::natural::Natural;
use malachite_q::Rational;

const DEBUG_FRACTIONAL_DIGITS: usize = 32;

#[derive(Clone, Eq, PartialEq)]
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
        $crate::Number::new_i64(-$value)
    };
    ($value:literal) => {
        $crate::Number::new_u64($value)
    };
    (- $numerator:literal / $denominator:literal) => {
        $crate::Number::new_ratio_i64(-$numerator, $denominator)
    };
    ($numerator:literal / $denominator:literal) => {
        $crate::Number::new_ratio_i64($numerator, $denominator)
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

macro_rules! impl_from_nonzero {
    ($type:ty, $constructor:ident) => {
        impl From<$type> for Number {
            fn from(value: $type) -> Self {
                Self::$constructor(value)
            }
        }
    };
}

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

#[cfg(test)]
mod tests {
    use super::Number;

    #[test]
    fn creates_from_primitive_integer() {
        assert_eq!(Number::new_i32(-7), Number::new_i64(-7));
    }

    #[test]
    fn creates_from_nonzero_integers() {
        const I8: core::num::NonZeroI8 = core::num::NonZeroI8::new(-1).unwrap();
        const I16: core::num::NonZeroI16 = core::num::NonZeroI16::new(-2).unwrap();
        const I32: core::num::NonZeroI32 = core::num::NonZeroI32::new(-3).unwrap();
        const I64: core::num::NonZeroI64 = core::num::NonZeroI64::new(-4).unwrap();
        const ISIZE: core::num::NonZeroIsize = core::num::NonZeroIsize::new(-5).unwrap();
        const U8: core::num::NonZeroU8 = core::num::NonZeroU8::new(1).unwrap();
        const U16: core::num::NonZeroU16 = core::num::NonZeroU16::new(2).unwrap();
        const U32: core::num::NonZeroU32 = core::num::NonZeroU32::new(3).unwrap();
        const U64: core::num::NonZeroU64 = core::num::NonZeroU64::new(4).unwrap();
        const USIZE: core::num::NonZeroUsize = core::num::NonZeroUsize::new(5).unwrap();

        const FROM_I8: Number = Number::new_nonzero_i8(I8);
        const FROM_I16: Number = Number::new_nonzero_i16(I16);
        const FROM_I32: Number = Number::new_nonzero_i32(I32);
        const FROM_I64: Number = Number::new_nonzero_i64(I64);
        const FROM_ISIZE: Number = Number::new_nonzero_isize(ISIZE);
        const FROM_U8: Number = Number::new_nonzero_u8(U8);
        const FROM_U16: Number = Number::new_nonzero_u16(U16);
        const FROM_U32: Number = Number::new_nonzero_u32(U32);
        const FROM_U64: Number = Number::new_nonzero_u64(U64);
        const FROM_USIZE: Number = Number::new_nonzero_usize(USIZE);

        assert_eq!(FROM_I8, Number::new_i8(-1));
        assert_eq!(FROM_I16, Number::new_i16(-2));
        assert_eq!(FROM_I32, Number::new_i32(-3));
        assert_eq!(FROM_I64, Number::new_i64(-4));
        assert_eq!(FROM_ISIZE, Number::new_isize(-5));
        assert_eq!(FROM_U8, Number::new_u8(1));
        assert_eq!(FROM_U16, Number::new_u16(2));
        assert_eq!(FROM_U32, Number::new_u32(3));
        assert_eq!(FROM_U64, Number::new_u64(4));
        assert_eq!(FROM_USIZE, Number::new_usize(5));

        let i128 = core::num::NonZeroI128::new(-6).unwrap();
        let u128 = core::num::NonZeroU128::new(6).unwrap();
        assert_eq!(Number::new_nonzero_i128(i128), Number::new_i128(-6));
        assert_eq!(Number::from(u128), Number::new_u128(6));
    }

    #[test]
    fn creates_from_num_macro_at_const_time() {
        const NEGATIVE: Number = num!(-1231232312311232123);
        const ZERO: Number = num!(0);
        const POSITIVE: Number = num!(123123123);
        const RATIO: Number = num!(32 / 12);

        assert_eq!(NEGATIVE, Number::new_i64(-1231232312311232123));
        assert_eq!(ZERO, Number::new_i64(0));
        assert_eq!(POSITIVE, Number::new_i64(123123123));
        assert_eq!(RATIO, Number::new_ratio_i64(32, 12));
    }

    #[test]
    fn debug_formats_rationals_as_decimal_numbers() {
        assert_eq!(format!("{:?}", Number::new_ratio_i64(1, 2)), "0.5");
        assert_eq!(
            format!("{:?}", Number::new_ratio_i64(32, 12)),
            "2.66666666666666666666666666666666..."
        );
        assert_eq!(
            format!(
                "{:?}",
                Number::new_ratio_i128(10031232131231312321, 10_000_000_000)
            ),
            "1003123213.1231312321"
        );
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_serializes_as_string() {
        let number = Number::new_ratio_i64(2, 3);
        let encoded = serde_json::to_string(&number).unwrap();

        assert_eq!(encoded, "\"2/3\"");
        assert_eq!(serde_json::from_str::<Number>(&encoded).unwrap(), number);
    }

    #[cfg(feature = "borsh")]
    #[test]
    fn borsh_serializes_as_bytes() {
        let number = Number::new_ratio_i64(2, 3);
        let encoded = borsh::to_vec(&number).unwrap();

        assert_eq!(&encoded[..4], 3u32.to_le_bytes().as_slice());
        assert_eq!(&encoded[4..], b"2/3");
        assert_eq!(borsh::from_slice::<Number>(&encoded).unwrap(), number);
    }

    #[cfg(feature = "num-bigint")]
    #[test]
    fn creates_from_num_bigint() {
        let bigint = num_bigint::BigInt::parse_bytes(b"-123456789123456789", 10).unwrap();
        let biguint = num_bigint::BigUint::parse_bytes(b"123456789123456789", 10).unwrap();

        assert_eq!(
            Number::new_num_bigint(bigint),
            Number::new_i128(-123456789123456789)
        );
        assert_eq!(Number::from(biguint), Number::new_u128(123456789123456789));
    }

    #[cfg(feature = "num-ration")]
    #[test]
    fn creates_from_num_rational() {
        let rational = num_rational::Ratio::new(-22i32, 7);

        assert_eq!(
            Number::new_num_rational(rational),
            Number::from(num_rational::Ratio::new(-44i32, 14))
        );
    }

    #[cfg(feature = "ruint")]
    #[test]
    fn creates_from_ruint() {
        let value = ruint::aliases::U256::from(123456789123456789u128);

        assert_eq!(
            Number::new_ruint(value),
            Number::new_u128(123456789123456789)
        );
    }
}
