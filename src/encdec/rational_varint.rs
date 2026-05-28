use malachite_base::num::arithmetic::traits::Sign;
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::Digits;
use malachite_nz::natural::Natural;
use malachite_q::Rational;

const VARINT_BASE: u8 = 128;

pub(crate) enum DecodeVarintError<E> {
    Read(E),
    InvalidVarint,
    ZeroDenominator,
}

pub(crate) fn size(value: &Rational) -> usize {
    let numerator = encode_signed_numerator(value);
    let denominator = value.to_denominator();
    varint_size(&numerator) + varint_size(&denominator)
}

pub(crate) fn encode(value: &Rational) -> Vec<u8> {
    let mut encoded = Vec::with_capacity(size(value));
    encode_unsigned_varint(&encode_signed_numerator(value), &mut encoded);
    encode_unsigned_varint(&value.to_denominator(), &mut encoded);
    encoded
}

pub(crate) fn decode_with<E, F>(mut read_byte: F) -> Result<Rational, DecodeVarintError<E>>
where
    F: FnMut() -> Result<u8, E>,
{
    let encoded_numerator = decode_unsigned_varint(&mut read_byte)?;
    let denominator = decode_unsigned_varint(&mut read_byte)?;

    if denominator == 0 {
        return Err(DecodeVarintError::ZeroDenominator);
    }

    let (sign, numerator) = decode_signed_numerator(encoded_numerator);
    Ok(Rational::from_sign_and_naturals(
        sign,
        numerator,
        denominator,
    ))
}

fn varint_size(value: &Natural) -> usize {
    let size = value.to_digits_asc(&VARINT_BASE).len();
    size.max(1)
}

fn encode_signed_numerator(value: &Rational) -> Natural {
    let numerator = value.to_numerator();
    if value.sign().is_lt() {
        (numerator << 1u32) - Natural::ONE
    } else {
        numerator << 1u32
    }
}

fn decode_signed_numerator(value: Natural) -> (bool, Natural) {
    if &value % Natural::from(2u8) == 0 {
        (true, value >> 1u32)
    } else {
        (false, (value + Natural::ONE) >> 1u32)
    }
}

fn encode_unsigned_varint(value: &Natural, dest: &mut Vec<u8>) {
    let mut digits = value.to_digits_asc(&VARINT_BASE);
    if digits.is_empty() {
        digits.push(0);
    }

    let last_index = digits.len() - 1;
    for (index, digit) in digits.into_iter().enumerate() {
        let byte = if index == last_index {
            digit
        } else {
            digit | 0b1000_0000
        };
        dest.push(byte);
    }
}

fn decode_unsigned_varint<E, F>(read_byte: &mut F) -> Result<Natural, DecodeVarintError<E>>
where
    F: FnMut() -> Result<u8, E>,
{
    let mut digits = Vec::new();
    loop {
        let byte = read_byte().map_err(DecodeVarintError::Read)?;
        digits.push(byte & 0b0111_1111);
        if byte & 0b1000_0000 == 0 {
            return Natural::from_digits_asc(&VARINT_BASE, digits.into_iter())
                .ok_or(DecodeVarintError::InvalidVarint);
        }
    }
}
