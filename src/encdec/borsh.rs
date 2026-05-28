use crate::{Number, rational_varint};

#[cfg(feature = "borsh__schema")]
use borsh::schema::{Declaration, Definition, Fields};

impl ::borsh::BorshSerialize for Number {
    fn serialize<W: ::borsh::io::Write>(&self, writer: &mut W) -> ::borsh::io::Result<()> {
        writer.write_all(&rational_varint::encode(&self.0))
    }
}

#[cfg(feature = "borsh__schema")]
impl ::borsh::BorshSchema for Number {
    fn add_definitions_recursively(
        definitions: &mut ::std::collections::BTreeMap<Declaration, Definition>,
    ) {
        definitions.insert(
            Self::declaration(),
            Definition::Struct {
                fields: Fields::UnnamedFields(vec![
                    "ZigZagVarint".to_owned(),
                    "UnsignedVarint".to_owned(),
                ]),
            },
        );
        let varint = Definition::Sequence {
            length_width: 0,
            length_range: 1..=u64::MAX,
            elements: u8::declaration(),
        };
        definitions.insert("ZigZagVarint".to_owned(), varint.clone());
        definitions.insert("UnsignedVarint".to_owned(), varint);
        u8::add_definitions_recursively(definitions);
    }

    fn declaration() -> Declaration {
        "Number".into()
    }
}

impl ::borsh::BorshDeserialize for Number {
    fn deserialize_reader<R: ::borsh::io::Read>(reader: &mut R) -> ::borsh::io::Result<Self> {
        rational_varint::decode_with(|| {
            let mut byte = [0];
            reader.read_exact(&mut byte).map(|()| byte[0])
        })
        .map(Self)
        .map_err(|error| match error {
            rational_varint::DecodeVarintError::Read(error) => error,
            rational_varint::DecodeVarintError::InvalidVarint => ::borsh::io::Error::new(
                ::borsh::io::ErrorKind::InvalidData,
                "invalid arbitrary-size varint",
            ),
            rational_varint::DecodeVarintError::ZeroDenominator => ::borsh::io::Error::new(
                ::borsh::io::ErrorKind::InvalidData,
                "rational denominator should not be zero",
            ),
        })
    }
}
