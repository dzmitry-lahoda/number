use crate::Number;

#[cfg(feature = "borsh__schema")]
use ::borsh::schema::{Declaration, Definition, Fields};

impl ::borsh::BorshSerialize for Number {
    fn serialize<W: ::borsh::io::Write>(&self, writer: &mut W) -> ::borsh::io::Result<()> {
        ::borsh::BorshSerialize::serialize(self.0.to_string().as_bytes(), writer)
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
                fields: Fields::UnnamedFields(vec![String::declaration()]),
            },
        );
        String::add_definitions_recursively(definitions);
    }

    fn declaration() -> Declaration {
        "Number".into()
    }
}

impl ::borsh::BorshDeserialize for Number {
    fn deserialize_reader<R: ::borsh::io::Read>(reader: &mut R) -> ::borsh::io::Result<Self> {
        let bytes = <Vec<u8> as ::borsh::BorshDeserialize>::deserialize_reader(reader)?;
        let value = core::str::from_utf8(&bytes)
            .map_err(|error| ::borsh::io::Error::new(::borsh::io::ErrorKind::InvalidData, error))?;
        value.parse().map_err(|()| {
            ::borsh::io::Error::new(::borsh::io::ErrorKind::InvalidData, "invalid rational")
        })
    }
}
