//! varint(7bit) zigzag of rational
use std::io::{self, BufRead, Error, ErrorKind, Write};

use malachite_base::num::arithmetic::traits::Sign;
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::Digits;
use malachite_nz::natural::Natural;
use malachite_q::Rational;

use crate::Number;

const VARINT_BASE: u8 = 128;

pub trait Serialize {
    fn size(&self) -> usize;

    fn serialize<T: Write>(&self, writer: T) -> io::Result<()>;
}

pub trait Deserialize: Sized {
    fn deserialize<T: BufRead>(reader: T) -> io::Result<Self>;
}

fn varint_size(value: &Natural) -> usize {
    let size = value.to_digits_asc(&VARINT_BASE).len();
    size.max(1)
}

fn serialize_unsigned_varint<T: Write>(value: &Natural, writer: &mut T) -> io::Result<()> {
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
        writer.write_all(&[byte])?;
    }
    Ok(())
}

fn deserialize_unsigned_varint<T: BufRead>(reader: &mut T) -> io::Result<Natural> {
    let mut digits = Vec::new();
    loop {
        let mut byte = [0; 1];
        reader.read_exact(&mut byte)?;
        digits.push(byte[0] & 0b0111_1111);
        if byte[0] & 0b1000_0000 == 0 {
            return Natural::from_digits_asc(&VARINT_BASE, digits.into_iter()).ok_or_else(|| {
                Error::new(ErrorKind::InvalidData, "invalid arbitrary-size varint")
            });
        }
    }
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

impl Serialize for Number {
    fn size(&self) -> usize {
        let numerator = encode_signed_numerator(&self.0);
        let denominator = self.0.to_denominator();
        varint_size(&numerator) + varint_size(&denominator)
    }

    fn serialize<T: Write>(&self, mut writer: T) -> io::Result<()> {
        serialize_unsigned_varint(&encode_signed_numerator(&self.0), &mut writer)?;
        serialize_unsigned_varint(&self.0.to_denominator(), &mut writer)
    }
}

impl Deserialize for Number {
    fn deserialize<T: BufRead>(mut reader: T) -> io::Result<Self> {
        let encoded_numerator = deserialize_unsigned_varint(&mut reader)?;
        let denominator = deserialize_unsigned_varint(&mut reader)?;

        if denominator == 0 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "rational denominator should not be zero",
            ));
        }

        if !reader.fill_buf()?.is_empty() {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "trailing bytes after rational varints",
            ));
        }

        let (sign, numerator) = decode_signed_numerator(encoded_numerator);
        Ok(Self(Rational::from_sign_and_naturals(
            sign,
            numerator,
            denominator,
        )))
    }
}
