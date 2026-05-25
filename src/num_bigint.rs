use crate::Number;

impl Number {
    pub fn new_num_bigint(value: ::num_bigint::BigInt) -> Self {
        Self(
            value
                .to_string()
                .parse()
                .expect("num value should parse as malachite rational"),
        )
    }

    pub fn new_num_biguint(value: ::num_bigint::BigUint) -> Self {
        Self(
            value
                .to_string()
                .parse()
                .expect("num value should parse as malachite rational"),
        )
    }
}

impl_lhs_ops!(::num_bigint::BigInt);
impl_lhs_ops!(::num_bigint::BigUint);

impl From<::num_bigint::BigInt> for Number {
    fn from(value: ::num_bigint::BigInt) -> Self {
        Self::new_num_bigint(value)
    }
}

impl From<::num_bigint::BigUint> for Number {
    fn from(value: ::num_bigint::BigUint) -> Self {
        Self::new_num_biguint(value)
    }
}
