use malachite_q::Rational;

#[derive(Clone, Debug, Eq, PartialEq)]
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

#[cfg(any(feature = "num-bigint", feature = "num-ration", feature = "ruint"))]
fn parse_rational(value: &str) -> Rational {
    value
        .parse()
        .expect("num value should parse as malachite rational")
}

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
