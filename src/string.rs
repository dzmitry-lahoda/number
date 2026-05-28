use core::fmt;
use core::str::FromStr;

use malachite_nz::natural::Natural;
use malachite_q::Rational;

use crate::Number;

const DEBUG_FRACTIONAL_DIGITS: usize = 32;

impl Number {
    pub(crate) fn parse_str(value: &str) -> Result<Self, ()> {
        if let Ok(value) = value.parse() {
            return Ok(Self(value));
        }

        parse_decimal_rational(value).map(Self)
    }
}

impl FromStr for Number {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse_str(value)
    }
}

//TODO: use https://github.com/mhogrefe/malachite/tree/master/malachite-q/src/conversion
// Convert finite decimal/scientific notation into an exact integer ratio.
fn parse_decimal_rational(value: &str) -> Result<Rational, ()> {
    let bytes = value.as_bytes();
    let mut index = 0;
    let negative = match bytes.first() {
        Some(b'-') => {
            index = 1;
            true
        }
        Some(b'+') => {
            index = 1;
            false
        }
        _ => false,
    };

    let mut digits = String::new();
    let mut fractional_digits = 0usize;
    let mut seen_digit = false;
    let mut seen_decimal_point = false;

    while index < bytes.len() {
        match bytes[index] {
            b'_' => {}
            b'.' => {
                if seen_decimal_point {
                    return Err(());
                }
                seen_decimal_point = true;
            }
            b'0'..=b'9' => {
                digits.push(char::from(bytes[index]));
                seen_digit = true;
                if seen_decimal_point {
                    fractional_digits = fractional_digits.checked_add(1).ok_or(())?;
                }
            }
            b'e' | b'E' => break,
            _ => return Err(()),
        }
        index += 1;
    }

    if !seen_digit || (!seen_decimal_point && index == bytes.len()) {
        return Err(());
    }

    let exponent = if index < bytes.len() {
        index += 1;
        parse_decimal_exponent(&bytes[index..])?
    } else {
        0
    };

    let scale = i128::try_from(fractional_digits)
        .map_err(|_| ())?
        .checked_sub(exponent)
        .ok_or(())?;
    let first_non_zero = digits.find(|digit| digit != '0');
    let mut numerator = match first_non_zero {
        Some(index) => digits[index..].to_owned(),
        None => String::from("0"),
    };

    if scale < 0 {
        let zeros = usize::try_from(-scale).map_err(|_| ())?;
        numerator.reserve(zeros);
        for _ in 0..zeros {
            numerator.push('0');
        }
    }

    if negative && numerator != "0" {
        numerator.insert(0, '-');
    }

    if scale <= 0 {
        return numerator.parse().map_err(|_| ());
    }

    let zeros = usize::try_from(scale).map_err(|_| ())?;
    let mut rational = numerator;
    rational.push('/');
    rational.push('1');
    rational.reserve(zeros);
    for _ in 0..zeros {
        rational.push('0');
    }
    rational.parse().map_err(|_| ())
}

fn parse_decimal_exponent(bytes: &[u8]) -> Result<i128, ()> {
    let mut index = 0;
    let negative = match bytes.first() {
        Some(b'-') => {
            index = 1;
            true
        }
        Some(b'+') => {
            index = 1;
            false
        }
        _ => false,
    };
    let mut exponent = 0i128;
    let mut seen_digit = false;

    while index < bytes.len() {
        match bytes[index] {
            b'_' => {}
            b'0'..=b'9' => {
                exponent = exponent
                    .checked_mul(10)
                    .and_then(|value| value.checked_add(i128::from(bytes[index] - b'0')))
                    .ok_or(())?;
                seen_digit = true;
            }
            _ => return Err(()),
        }
        index += 1;
    }

    if !seen_digit {
        return Err(());
    }

    if negative {
        exponent.checked_neg().ok_or(())
    } else {
        Ok(exponent)
    }
}

impl fmt::Debug for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rational = self.0.to_string();
        if f.alternate() {
            return f.write_str(&rational);
        }

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
