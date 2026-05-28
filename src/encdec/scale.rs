use crate::Number;
use crate::rational_varint;

impl ::parity_scale_codec::Encode for Number {
    fn size_hint(&self) -> usize {
        rational_varint::size(&self.0)
    }

    fn encode_to<T: ::parity_scale_codec::Output + ?Sized>(&self, dest: &mut T) {
        dest.write(&rational_varint::encode(&self.0));
    }
}

impl ::parity_scale_codec::Decode for Number {
    fn decode<I: ::parity_scale_codec::Input>(
        input: &mut I,
    ) -> Result<Self, ::parity_scale_codec::Error> {
        rational_varint::decode_with(|| input.read_byte())
            .map(Self)
            .map_err(|error| match error {
                rational_varint::DecodeVarintError::Read(error) => error,
                rational_varint::DecodeVarintError::InvalidVarint => {
                    "invalid arbitrary-size varint".into()
                }
                rational_varint::DecodeVarintError::ZeroDenominator => {
                    "rational denominator should not be zero".into()
                }
            })
    }
}
