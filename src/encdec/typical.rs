//! varint(7bit) zigzag of rational
use std::io::{self, BufRead, Error, ErrorKind, Write};

use crate::{Number, rational_varint};

pub trait Serialize {
    fn size(&self) -> usize;

    fn serialize<T: Write>(&self, writer: T) -> io::Result<()>;
}

pub trait Deserialize: Sized {
    fn deserialize<T: BufRead>(reader: T) -> io::Result<Self>;
}

impl Serialize for Number {
    fn size(&self) -> usize {
        rational_varint::size(&self.0)
    }

    fn serialize<T: Write>(&self, mut writer: T) -> io::Result<()> {
        writer.write_all(&rational_varint::encode(&self.0))
    }
}

impl Deserialize for Number {
    fn deserialize<T: BufRead>(mut reader: T) -> io::Result<Self> {
        let rational = rational_varint::decode_with(|| {
            let mut byte = [0; 1];
            reader.read_exact(&mut byte).map(|()| byte[0])
        })
        .map_err(|error| match error {
            rational_varint::DecodeVarintError::Read(error) => error,
            rational_varint::DecodeVarintError::InvalidVarint => {
                Error::new(ErrorKind::InvalidData, "invalid arbitrary-size varint")
            }
            rational_varint::DecodeVarintError::ZeroDenominator => Error::new(
                ErrorKind::InvalidData,
                "rational denominator should not be zero",
            ),
        })?;
        if !reader.fill_buf()?.is_empty() {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "trailing bytes after rational varints",
            ));
        }

        Ok(Self(rational))
    }
}
