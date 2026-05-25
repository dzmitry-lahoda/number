use crate::Number;

impl Number {
    pub fn new_ruint<const BITS: usize, const LIMBS: usize>(
        value: ::ruint::Uint<BITS, LIMBS>,
    ) -> Self {
        Self(
            value
                .to_string()
                .parse()
                .expect("num value should parse as malachite rational"),
        )
    }
}

impl<const BITS: usize, const LIMBS: usize> From<::ruint::Uint<BITS, LIMBS>> for Number {
    fn from(value: ::ruint::Uint<BITS, LIMBS>) -> Self {
        Self::new_ruint(value)
    }
}
