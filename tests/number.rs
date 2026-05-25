use number::{Number, num};

#[test]
fn creates_from_primitive_integer() {
    assert_eq!(Number::new_i32(-7), Number::new_i64(-7));
}

#[test]
fn supports_ordering_and_hashing() {
    let mut values = [Number::new_i64(2), Number::new_ratio_i64(1, 2)];
    values.sort();
    assert_eq!(values, [Number::new_ratio_i64(1, 2), Number::new_i64(2)]);

    let mut set = std::collections::HashSet::new();
    set.insert(Number::new_ratio_i64(2, 4));
    assert!(set.contains(&Number::new_ratio_i64(1, 2)));
}

#[test]
fn supports_math_ops_with_convertible_numbers() {
    let one_half = Number::new_ratio_i64(1, 2);

    assert_eq!(one_half.clone() + 2i32, Number::new_ratio_i64(5, 2));
    assert_eq!(&one_half + 2u64, Number::new_ratio_i64(5, 2));
    assert_eq!(2i32 + one_half.clone(), Number::new_ratio_i64(5, 2));
    assert_eq!(2i32 - &one_half, Number::new_ratio_i64(3, 2));
    assert_eq!(2i32 * one_half.clone(), Number::new_i64(1));
    assert_eq!(2i32 / &one_half, Number::new_i64(4));
    assert_eq!(Number::new_i64(5) % 2u8, Number::new_i64(1));
    assert_eq!(-&one_half, Number::new_ratio_i64(-1, 2));

    let nonzero = core::num::NonZeroI32::new(3).unwrap();
    assert_eq!(nonzero + one_half, Number::new_ratio_i64(7, 2));
}

#[test]
fn raises_to_integer_powers() {
    let two_thirds = Number::new_ratio_i64(2, 3);

    assert_eq!(two_thirds.clone().pow(3i16), Number::new_ratio_i64(8, 27));
    assert_eq!(two_thirds.clone().pow(-2i16), Number::new_ratio_i64(9, 4));
    assert_eq!(two_thirds.pow(0i16), Number::new_i64(1));
}

#[test]
fn sums_iterators() {
    let values = [
        Number::new_ratio_i64(1, 2),
        Number::new_ratio_i64(1, 3),
        Number::new_i64(2),
    ];

    assert_eq!(
        values.clone().into_iter().sum::<Number>(),
        Number::new_ratio_i64(17, 6)
    );
    assert_eq!(values.iter().sum::<Number>(), Number::new_ratio_i64(17, 6));
    assert_eq!(
        std::iter::empty::<Number>().sum::<Number>(),
        Number::new_i64(0)
    );
}

#[test]
fn creates_from_nonzero_integers() {
    const I8: core::num::NonZeroI8 = core::num::NonZeroI8::new(-1).unwrap();
    const I16: core::num::NonZeroI16 = core::num::NonZeroI16::new(-2).unwrap();
    const I32: core::num::NonZeroI32 = core::num::NonZeroI32::new(-3).unwrap();
    const I64: core::num::NonZeroI64 = core::num::NonZeroI64::new(-4).unwrap();
    const ISIZE: core::num::NonZeroIsize = core::num::NonZeroIsize::new(-5).unwrap();
    const U8: core::num::NonZeroU8 = core::num::NonZeroU8::new(1).unwrap();
    const U16: core::num::NonZeroU16 = core::num::NonZeroU16::new(2).unwrap();
    const U32: core::num::NonZeroU32 = core::num::NonZeroU32::new(3).unwrap();
    const U64: core::num::NonZeroU64 = core::num::NonZeroU64::new(4).unwrap();
    const USIZE: core::num::NonZeroUsize = core::num::NonZeroUsize::new(5).unwrap();

    const FROM_I8: Number = Number::new_nonzero_i8(I8);
    const FROM_I16: Number = Number::new_nonzero_i16(I16);
    const FROM_I32: Number = Number::new_nonzero_i32(I32);
    const FROM_I64: Number = Number::new_nonzero_i64(I64);
    const FROM_ISIZE: Number = Number::new_nonzero_isize(ISIZE);
    const FROM_U8: Number = Number::new_nonzero_u8(U8);
    const FROM_U16: Number = Number::new_nonzero_u16(U16);
    const FROM_U32: Number = Number::new_nonzero_u32(U32);
    const FROM_U64: Number = Number::new_nonzero_u64(U64);
    const FROM_USIZE: Number = Number::new_nonzero_usize(USIZE);

    assert_eq!(FROM_I8, Number::new_i8(-1));
    assert_eq!(FROM_I16, Number::new_i16(-2));
    assert_eq!(FROM_I32, Number::new_i32(-3));
    assert_eq!(FROM_I64, Number::new_i64(-4));
    assert_eq!(FROM_ISIZE, Number::new_isize(-5));
    assert_eq!(FROM_U8, Number::new_u8(1));
    assert_eq!(FROM_U16, Number::new_u16(2));
    assert_eq!(FROM_U32, Number::new_u32(3));
    assert_eq!(FROM_U64, Number::new_u64(4));
    assert_eq!(FROM_USIZE, Number::new_usize(5));

    let i128 = core::num::NonZeroI128::new(-6).unwrap();
    let u128 = core::num::NonZeroU128::new(6).unwrap();
    assert_eq!(Number::new_nonzero_i128(i128), Number::new_i128(-6));
    assert_eq!(Number::from(u128), Number::new_u128(6));
}

#[test]
fn creates_from_num_macro_at_const_time() {
    const NEGATIVE: Number = num!(-1231232312311232123);
    const ZERO: Number = num!(0);
    const POSITIVE: Number = num!(123123123);
    const RATIO: Number = num!(32 / 12);
    const NEGATIVE_RATIO: Number = num!(-32 / 12);
    const NEGATIVE_DENOMINATOR_RATIO: Number = num!(32 / -12);
    const POSITIVE_FROM_TWO_NEGATIVES_RATIO: Number = num!(-32 / -12);
    const DECIMAL_NUMERATOR_RATIO: Number = num!(42.21 / 3);
    const DECIMAL_DENOMINATOR_RATIO: Number = num!(1 / 2.5);
    const DECIMAL_RATIO: Number = num!(1.5 / 0.25);
    const DECIMAL: Number = num!(42.21);
    const NEGATIVE_DECIMAL: Number = num!(-42.21);
    const UNDERSCORED_DECIMAL: Number = num!(1_000.050);

    assert_eq!(NEGATIVE, Number::new_i64(-1231232312311232123));
    assert_eq!(ZERO, Number::new_i64(0));
    assert_eq!(POSITIVE, Number::new_i64(123123123));
    assert_eq!(RATIO, Number::new_ratio_i64(32, 12));
    assert_eq!(NEGATIVE_RATIO, Number::new_ratio_i64(-32, 12));
    assert_eq!(NEGATIVE_DENOMINATOR_RATIO, Number::new_ratio_i64(-32, 12));
    assert_eq!(
        POSITIVE_FROM_TWO_NEGATIVES_RATIO,
        Number::new_ratio_i64(32, 12)
    );
    assert_eq!(DECIMAL_NUMERATOR_RATIO, Number::new_ratio_i64(4221, 300));
    assert_eq!(DECIMAL_DENOMINATOR_RATIO, Number::new_ratio_i64(10, 25));
    assert_eq!(DECIMAL_RATIO, Number::new_ratio_i64(1500, 250));
    assert_eq!(DECIMAL, Number::new_ratio_i64(4221, 100));
    assert_eq!(NEGATIVE_DECIMAL, Number::new_ratio_i64(-4221, 100));
    assert_eq!(UNDERSCORED_DECIMAL, Number::new_ratio_i64(1_000_050, 1000));
}

#[test]
fn debug_formats_rationals_as_decimal_numbers() {
    assert_eq!(format!("{:?}", Number::new_ratio_i64(1, 2)), "0.5");
    assert_eq!(
        format!("{:?}", Number::new_ratio_i64(32, 12)),
        "2.66666666666666666666666666666666..."
    );
    assert_eq!(
        format!(
            "{:?}",
            Number::new_ratio_i128(10031232131231312321, 10_000_000_000)
        ),
        "1003123213.1231312321"
    );
}

#[cfg(feature = "serde")]
#[test]
fn serde_serializes_as_string() {
    let number = Number::new_ratio_i64(2, 3);
    let encoded = serde_json::to_string(&number).unwrap();

    assert_eq!(encoded, "\"2/3\"");
    assert_eq!(serde_json::from_str::<Number>(&encoded).unwrap(), number);
}

#[cfg(feature = "borsh")]
#[test]
fn borsh_serializes_as_bytes() {
    let number = Number::new_ratio_i64(2, 3);
    let encoded = borsh::to_vec(&number).unwrap();

    assert_eq!(&encoded[..4], 3u32.to_le_bytes().as_slice());
    assert_eq!(&encoded[4..], b"2/3");
    assert_eq!(borsh::from_slice::<Number>(&encoded).unwrap(), number);
}

#[cfg(feature = "num-bigint")]
#[test]
fn creates_from_num_bigint() {
    let bigint = num_bigint::BigInt::parse_bytes(b"-123456789123456789", 10).unwrap();
    let biguint = num_bigint::BigUint::parse_bytes(b"123456789123456789", 10).unwrap();

    assert_eq!(
        Number::new_num_bigint(bigint),
        Number::new_i128(-123456789123456789)
    );
    assert_eq!(Number::from(biguint), Number::new_u128(123456789123456789));
}

#[cfg(feature = "num-bigint")]
#[test]
fn supports_math_ops_with_num_bigint() {
    let bigint = num_bigint::BigInt::from(2);
    let biguint = num_bigint::BigUint::from(2u32);
    let one_half = Number::new_ratio_i64(1, 2);

    assert_eq!(Number::new_i64(2) + bigint.clone(), Number::new_i64(4));
    assert_eq!(bigint + &one_half, Number::new_ratio_i64(5, 2));
    assert_eq!(biguint * one_half, Number::new_i64(1));
}

#[cfg(feature = "num-traits")]
#[test]
fn supports_num_traits() {
    use num_traits::{Inv, Num, One, Pow, Signed, Zero};

    assert!(Number::zero().is_zero());
    assert!(Number::one().is_one());
    assert_eq!(
        <Number as Num>::from_str_radix("22/7", 10).unwrap(),
        Number::new_ratio_i64(22, 7)
    );
    assert!(<Number as Num>::from_str_radix("10", 2).is_err());
    assert_eq!(Number::new_i64(-2).abs(), Number::new_i64(2));
    assert_eq!(
        Number::new_i64(2).abs_sub(&Number::new_i64(5)),
        Number::new_i64(0)
    );
    assert_eq!(Number::new_i64(-2).signum(), Number::new_i64(-1));
    assert_eq!(Number::new_i64(2).inv(), Number::new_ratio_i64(1, 2));
    assert_eq!(
        Pow::pow(Number::new_ratio_i64(2, 3), 3i16),
        Number::new_ratio_i64(8, 27)
    );
    assert_eq!(
        Pow::pow(&Number::new_ratio_i64(2, 3), -2i16),
        Number::new_ratio_i64(9, 4)
    );
}

#[cfg(feature = "num-ration")]
#[test]
fn creates_from_num_rational() {
    let rational = num_rational::Ratio::new(-22i32, 7);

    assert_eq!(
        Number::new_num_rational(rational),
        Number::from(num_rational::Ratio::new(-44i32, 14))
    );
}

#[cfg(feature = "ruint")]
#[test]
fn creates_from_ruint() {
    let value = ruint::aliases::U256::from(123456789123456789u128);

    assert_eq!(
        Number::new_ruint(value),
        Number::new_u128(123456789123456789)
    );
}
