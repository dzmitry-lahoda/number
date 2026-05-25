use number::{Number, num};

fn main() {
    construction_formats();
    formulas();

    #[cfg(feature = "borsh")]
    borsh_feature();
    #[cfg(feature = "num-bigint")]
    num_bigint_feature();
    #[cfg(feature = "num-rational")]
    num_rational_feature();
    #[cfg(feature = "num-traits")]
    num_traits_feature();
    #[cfg(feature = "ruint")]
    ruint_feature();
    #[cfg(feature = "schemars")]
    schemars_feature();
    #[cfg(feature = "serde")]
    serde_feature();
}

fn construction_formats() {
    const INTEGER: Number = num!(42);
    const NEGATIVE: Number = num!(-42);
    const DECIMAL: Number = num!(42.21);
    const RATIO: Number = num!(32 / 12);
    const DECIMAL_RATIO: Number = num!(1.5 / 0.25);

    assert_eq!(INTEGER, Number::new_i64(42));
    assert_eq!(NEGATIVE, Number::new_i64(-42));
    assert_eq!(DECIMAL, Number::new_ratio_i64(4221, 100));
    assert_eq!(RATIO, Number::new_ratio_i64(32, 12));
    assert_eq!(DECIMAL_RATIO, Number::new_i64(6));

    let primitive = 123i32;
    let numerator = 7i64;
    let denominator = 4i64;
    let maybe = Some(false);

    assert_eq!(num!(primitive), Number::new_i64(123));
    assert_eq!(num!(primitive + 1), Number::new_i64(124));
    assert_eq!(num!(numerator / denominator), Number::new_ratio_i64(7, 4));
    assert_eq!(num!(maybe), Number::new_i64(-1));
    assert_eq!(num!(None::<bool>), Number::new_i64(0));
}

fn formulas() {
    let input = 44u64;
    let fee_enabled = true;
    let numerator = 9i64;
    let denominator = 8i64;
    let adjusted_numerator = numerator + 3;
    let adjusted_denominator = denominator - 2;

    let weighted = num!(input) / 3 - u128::MAX * num!(2 / 3).pow(3i16);
    let signed_adjustment = fee_enabled * num!(5 / 2) + Some(false);
    let ratio_from_variables = num!(numerator / denominator);
    let adjusted_ratio = num!(adjusted_numerator / adjusted_denominator);
    let composed = (weighted + signed_adjustment) * &ratio_from_variables / &adjusted_ratio;

    assert_eq!(ratio_from_variables, Number::new_ratio_i64(9, 8));
    assert_eq!(adjusted_ratio, Number::new_i64(2));
    assert_eq!(
        format!("{:#?}", composed),
        "-1814839290245005138471331239636097127469/32"
    );
}

#[cfg(feature = "borsh")]
fn borsh_feature() {
    let value = num!(22 / 7);
    let encoded = borsh::to_vec(&value).unwrap();

    assert_eq!(borsh::from_slice::<Number>(&encoded).unwrap(), value);
}

#[cfg(feature = "num-bigint")]
fn num_bigint_feature() {
    let bigint = num_bigint::BigInt::from(-123456789i64);
    let biguint = num_bigint::BigUint::from(123456789u64);

    assert_eq!(num!(bigint), Number::new_i64(-123456789));
    assert_eq!(num!(biguint), Number::new_u64(123456789));
}

#[cfg(feature = "num-rational")]
fn num_rational_feature() {
    let ratio = num_rational::Ratio::new(22i64, 7);

    assert_eq!(num!(ratio), Number::new_ratio_i64(22, 7));
}

#[cfg(feature = "num-traits")]
fn num_traits_feature() {
    use num_traits::{One, Pow, Signed, Zero};

    assert_eq!(Number::zero(), num!(0));
    assert_eq!(Number::one().signum(), num!(1));
    assert_eq!(Pow::pow(num!(2 / 3), -2i16), Number::new_ratio_i64(9, 4));
}

#[cfg(feature = "ruint")]
fn ruint_feature() {
    let value = ruint::aliases::U256::from(123456789u64);

    assert_eq!(num!(value), Number::new_u64(123456789));
}

#[cfg(feature = "schemars")]
fn schemars_feature() {
    let schema = schemars::schema_for!(Number);
    let schema = serde_json::to_value(schema).unwrap();

    assert_eq!(schema["type"], "string");
}

#[cfg(feature = "serde")]
fn serde_feature() {
    let value = num!(22 / 7);
    let encoded = serde_json::to_string(&value).unwrap();

    assert_eq!(serde_json::from_str::<Number>(&encoded).unwrap(), value);
}
