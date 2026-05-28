use crate::Number;

impl ::borsh::BorshSerialize for Number {
    fn serialize<W: ::borsh::io::Write>(&self, writer: &mut W) -> ::borsh::io::Result<()> {
        ::borsh::BorshSerialize::serialize(self.0.to_string().as_bytes(), writer)
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
