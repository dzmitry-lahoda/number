#![allow(clippy::erasing_op)]

use number::{Number, num};

#[test]
fn creates_from_primitive_integer() {
    assert_eq!(Number::new_i32(-7), Number::new_i64(-7));
}

#[test]
fn creates_from_boolean_state() {
    assert_eq!(Number::from(true), num!(1));
    assert_eq!(Number::from(false), num!(-1));
    assert_eq!(Number::from(Some(true)), num!(1));
    assert_eq!(Number::from(Some(false)), num!(-1));
    assert_eq!(Number::from(None::<bool>), num!(0));
}

#[test]
fn parses_finite_decimal_strings_exactly() {
    assert_eq!("10.5".parse::<Number>().unwrap(), Number::from(21) / 2);
    assert_eq!(
        "-.125".parse::<Number>().unwrap(),
        Number::new_ratio_i64(-1, 8)
    );
    assert_eq!("1.25e2".parse::<Number>().unwrap(), Number::from(125));
    assert_eq!(
        "1e-3".parse::<Number>().unwrap(),
        Number::new_ratio_i64(1, 1000)
    );
}

#[cfg(feature = "float")]
#[test]
fn creates_from_finite_floats() {
    let half = 0.5f64;

    assert_eq!(Number::new_f32(0.5f32), Number::new_ratio_i64(1, 2));
    assert_eq!(Number::new_f64(-1.25f64), Number::new_ratio_i64(-5, 4));
    assert_eq!(
        Number::new_f32(0.1f32),
        Number::new_ratio_i64(13_421_773, 134_217_728)
    );
    assert_eq!(
        Number::new_f64(0.1f64),
        Number::new_ratio_i128(3_602_879_701_896_397, 36_028_797_018_963_968)
    );
    assert_eq!(num!(half), Number::new_ratio_i64(1, 2));
}

#[cfg(feature = "float")]
#[test]
fn rejects_non_finite_floats() {
    assert!(Number::try_new_f32(f32::NAN).is_err());
    assert!(Number::try_new_f64(f64::INFINITY).is_err());
    assert!(Number::try_new_f64(f64::NEG_INFINITY).is_err());
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
fn compares_signed_integer_samples() {
    use core::cmp::Ordering::{Equal, Greater, Less};

    let samples = [
        (-1, -1, Equal),
        (-1, 0, Less),
        (-1, 1, Less),
        (0, -1, Greater),
        (0, 0, Equal),
        (0, 1, Less),
        (1, -1, Greater),
        (1, 0, Greater),
        (1, 1, Equal),
        (999, 1000, Less),
        (1000, 1000, Equal),
        (1001, 1000, Greater),
        (1000, 999, Greater),
        (1000, 1001, Less),
        (-999, -1000, Greater),
        (-1000, -1000, Equal),
        (-1001, -1000, Less),
        (-1000, -999, Less),
        (-1000, -1001, Greater),
    ];

    for (left, right, result) in samples {
        assert_eq!(
            Number::new_i64(left).cmp(&Number::new_i64(right)),
            result,
            "{left:?} {} {right:?}",
            match result {
                Less => "<",
                Equal => "=",
                Greater => ">",
            }
        );
    }
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

    assert_eq!(true + Number::new_i64(2), Number::new_i64(3));
    assert_eq!(false * Number::new_i64(2), Number::new_i64(-2));
    assert_eq!(Some(true) - Number::new_i64(2), Number::new_i64(-1));
    assert_eq!(None::<bool> + Number::new_i64(2), Number::new_i64(2));
}

#[cfg(feature = "float")]
#[test]
fn supports_math_ops_with_floats() {
    assert_eq!(0.5f32 + Number::new_i64(2), Number::new_ratio_i64(5, 2));
    assert_eq!(2.5f64 * Number::new_i64(2), Number::new_i64(5));
    assert_eq!(Number::new_i64(1) / 0.5f64, Number::new_i64(2));
}

#[test]
fn supports_signed_integer_math_identities() {
    assert_eq!(num!(1) + 2, num!(3));
    assert_eq!(2 + num!(1), num!(3));
    assert_eq!(num!(-1) + -2, num!(-3));
    assert_eq!(-2 + num!(-1), num!(-3));
    assert_eq!(num!(-1) + 2, num!(1));
    assert_eq!(num!(-2) + 1, num!(-1));
    assert_eq!(num!(2) - 1, num!(1));
    assert_eq!(num!(1) - 2, num!(-1));
    assert_eq!(num!(0) - 0, num!(0));
    assert_eq!(num!(1) - 1, num!(0));
    assert_eq!(num!(1) + -1, num!(0));
    assert_eq!(num!(0) * -1, num!(0));
    assert_eq!(num!(1) * 0, num!(0));
    assert_eq!(num!(3) * 2, num!(6));
    assert_eq!(num!(-3) * -2, num!(6));
    assert_eq!(num!(3) * -2, num!(-6));
    assert_eq!(num!(-3) * 2, num!(-6));
}

#[test]
fn raises_to_integer_powers() {
    let two_thirds = Number::new_ratio_i64(2, 3);

    assert_eq!(two_thirds.clone().pow(3i16), Number::new_ratio_i64(8, 27));
    assert_eq!(two_thirds.clone().pow(-2i16), Number::new_ratio_i64(9, 4));
    assert_eq!(two_thirds.pow(0i16), Number::new_i64(1));

    assert_eq!(format!("{:?}", num!(2).pow(3i16)), "8");
    assert_eq!(format!("{:?}", num!(-2).pow(3i16)), "-8");
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
fn tries_into_primitive_integers() {
    macro_rules! assert_try_from_number {
        ($type:ty, $constructor:ident, $value:expr) => {{
            let value: $type = $value;
            let number = Number::$constructor(value);

            assert_eq!(<$type>::try_from(number.clone()).unwrap(), value);
            assert_eq!(<$type>::try_from(&number).unwrap(), value);
        }};
    }

    assert_try_from_number!(i8, new_i8, i8::MIN);
    assert_try_from_number!(i16, new_i16, i16::MIN);
    assert_try_from_number!(i32, new_i32, i32::MIN);
    assert_try_from_number!(i64, new_i64, i64::MIN);
    assert_try_from_number!(i128, new_i128, i128::MIN);
    assert_try_from_number!(isize, new_isize, isize::MIN);
    assert_try_from_number!(u8, new_u8, u8::MAX);
    assert_try_from_number!(u16, new_u16, u16::MAX);
    assert_try_from_number!(u32, new_u32, u32::MAX);
    assert_try_from_number!(u64, new_u64, u64::MAX);
    assert_try_from_number!(u128, new_u128, u128::MAX);
    assert_try_from_number!(usize, new_usize, usize::MAX);
}

#[test]
fn rejects_invalid_primitive_integer_conversions() {
    assert!(i8::try_from(Number::new_i64(i64::from(i8::MAX) + 1)).is_err());
    assert!(u8::try_from(Number::new_i64(-1)).is_err());
    assert!(u128::try_from(Number::new_i64(-1)).is_err());
    assert!(i128::try_from(Number::new_u128(u128::MAX)).is_err());
    assert!(i64::try_from(Number::new_ratio_i64(3, 2)).is_err());
}

#[test]
fn tries_into_nonzero_integers() {
    macro_rules! assert_try_from_number {
        ($type:ty, $constructor:ident, $value:expr) => {{
            let value: $type = <$type>::new($value).unwrap();
            let number = Number::$constructor(value);

            assert_eq!(<$type>::try_from(number.clone()).unwrap(), value);
            assert_eq!(<$type>::try_from(&number).unwrap(), value);
        }};
    }

    assert_try_from_number!(core::num::NonZeroI8, new_nonzero_i8, -1);
    assert_try_from_number!(core::num::NonZeroI16, new_nonzero_i16, -2);
    assert_try_from_number!(core::num::NonZeroI32, new_nonzero_i32, -3);
    assert_try_from_number!(core::num::NonZeroI64, new_nonzero_i64, -4);
    assert_try_from_number!(core::num::NonZeroI128, new_nonzero_i128, i128::MIN);
    assert_try_from_number!(core::num::NonZeroIsize, new_nonzero_isize, -5);
    assert_try_from_number!(core::num::NonZeroU8, new_nonzero_u8, 1);
    assert_try_from_number!(core::num::NonZeroU16, new_nonzero_u16, 2);
    assert_try_from_number!(core::num::NonZeroU32, new_nonzero_u32, 3);
    assert_try_from_number!(core::num::NonZeroU64, new_nonzero_u64, 4);
    assert_try_from_number!(core::num::NonZeroU128, new_nonzero_u128, u128::MAX);
    assert_try_from_number!(core::num::NonZeroUsize, new_nonzero_usize, 5);

    assert!(core::num::NonZeroU8::try_from(Number::new_i64(0)).is_err());
    assert!(core::num::NonZeroI8::try_from(Number::new_ratio_i64(1, 2)).is_err());
}

#[test]
fn creates_from_128_bit_integer_boundaries() {
    const I64_MIN_AS_NUMBER: Number = Number::new_i64(i64::MIN);
    const U64_MAX_AS_NUMBER: Number = Number::new_u64(u64::MAX);

    assert_eq!(I64_MIN_AS_NUMBER, Number::new_i128(i64::MIN as i128));
    assert_eq!(U64_MAX_AS_NUMBER, Number::new_u128(u64::MAX as u128));
    assert_eq!(
        format!("{:#?}", Number::new_i128(i128::MIN)),
        i128::MIN.to_string()
    );
    assert_eq!(
        format!("{:#?}", Number::new_i128(i128::MAX)),
        i128::MAX.to_string()
    );
    assert_eq!(
        format!("{:#?}", Number::new_u128(u128::MAX)),
        u128::MAX.to_string()
    );

    let nonzero_i128 = core::num::NonZeroI128::new(i128::MIN).unwrap();
    let nonzero_u128 = core::num::NonZeroU128::new(u128::MAX).unwrap();
    assert_eq!(
        Number::new_nonzero_i128(nonzero_i128),
        Number::new_i128(i128::MIN)
    );
    assert_eq!(
        Number::new_nonzero_u128(nonzero_u128),
        Number::new_u128(u128::MAX)
    );
}

#[test]
fn supports_128_bit_ratios() {
    let denominator = 1i128 << 100;
    let numerator = denominator * 3;

    assert_eq!(
        Number::new_ratio_i128(numerator, denominator),
        Number::new_i64(3)
    );
    assert_eq!(
        format!("{:#?}", Number::new_ratio_i128(i128::MIN, i128::MAX)),
        format!("{}/{}", i128::MIN, i128::MAX)
    );
    assert_eq!(
        format!("{:#?}", Number::new_ratio_i128(i128::MAX - 1, i128::MAX)),
        format!("{}/{}", i128::MAX - 1, i128::MAX)
    );
}

#[test]
fn supports_math_ops_with_128_bit_integers() {
    let big_u = u128::MAX;
    let big_i = i128::MIN + 1;
    let half = Number::new_ratio_i64(1, 2);

    assert_eq!(
        format!("{:#?}", half.clone() + big_u),
        "680564733841876926926749214863536422911/2"
    );
    assert_eq!(
        format!("{:#?}", big_u - half.clone()),
        "680564733841876926926749214863536422909/2"
    );
    assert_eq!(half.clone() * big_i, Number::new_ratio_i128(big_i, 2));
    assert_eq!(
        format!("{:#?}", big_i / half),
        "-340282366920938463463374607431768211454"
    );
}

#[test]
fn creates_from_num_macro_at_const_time() {
    const NEGATIVE: Number = num!(-1231232312311232123);
    const I64_MIN: Number = num!(-9223372036854775808);
    const ZERO: Number = num!(0);
    const POSITIVE: Number = num!(123123123);
    const U64_MAX: Number = num!(18446744073709551615);
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
    assert_eq!(I64_MIN, Number::new_i64(i64::MIN));
    assert_eq!(ZERO, Number::new_i64(0));
    assert_eq!(POSITIVE, Number::new_i64(123123123));
    assert_eq!(U64_MAX, Number::new_u64(u64::MAX));
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
fn creates_from_num_macro_with_convertible_expressions() {
    let two = 2i32;
    let numerator = 6i64;
    let denominator = 4i64;
    let adjusted_numerator = numerator + 2;
    let adjusted_denominator = denominator - 2;
    let big = u128::MAX;

    assert_eq!(num!(two), Number::new_i64(2));
    assert_eq!(num!(two + 3), Number::new_i64(5));
    assert_eq!(num!(Some(false)), Number::new_i64(-1));
    assert_eq!(num!(big), Number::new_u128(u128::MAX));
    assert_eq!(num!(numerator / denominator), Number::new_ratio_i64(3, 2));
    assert_eq!(num!(-numerator / denominator), Number::new_ratio_i64(-3, 2));
    assert_eq!(
        num!(adjusted_numerator / adjusted_denominator),
        Number::new_i64(4)
    );
}

#[test]
fn debug_formats_rationals_as_decimal_numbers() {
    assert_eq!(format!("{:?}", num!(-1)), "-1");
    assert_eq!(format!("{:?}", num!(3)), "3");
    assert_eq!(format!("{:?}", Number::new_i64(4) / 2), "2");
    assert_eq!(format!("{:?}", Number::new_ratio_i64(1, 2)), "0.5");
    assert_eq!(format!("{:#?}", Number::new_ratio_i64(1, 2)), "1/2");
    assert_eq!(format!("{:#?}", Number::new_ratio_i64(4, 2)), "2");
    assert_eq!(format!("{:?}", Number::new_i64(7) / 4), "1.75");
    assert_eq!(format!("{:?}", Number::new_i64(15) / 10), "1.5");
    assert_eq!(format!("{:?}", Number::new_i64(105) / 100), "1.05");
    assert_eq!(format!("{:?}", Number::new_i64(1) / 10), "0.1");
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

#[test]
fn zero_divided_by_integer_stays_zero() {
    assert_eq!(Number::new_i64(0) / 10_000, num!(0));
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

#[cfg(feature = "typical")]
#[test]
fn typical_serializes_as_two_arbitrary_size_varints() {
    use number::typical::{Deserialize, Serialize};

    let number = Number::new_ratio_i64(2, 3);
    let mut encoded = Vec::new();
    number.serialize(&mut encoded).unwrap();

    assert_eq!(encoded, vec![4, 3]);
    assert_eq!(number.size(), encoded.len());
    assert_eq!(
        Number::deserialize(std::io::Cursor::new(encoded)).unwrap(),
        number
    );

    let integer = Number::new_i64(42);
    let mut encoded = Vec::new();
    integer.serialize(&mut encoded).unwrap();
    assert_eq!(encoded, vec![84, 1]);
    assert_eq!(
        Number::deserialize(std::io::Cursor::new(encoded)).unwrap(),
        integer
    );

    let negative = Number::new_ratio_i64(-2, 3);
    let mut encoded = Vec::new();
    negative.serialize(&mut encoded).unwrap();
    assert_eq!(encoded, vec![3, 3]);
    assert_eq!(
        Number::deserialize(std::io::Cursor::new(encoded)).unwrap(),
        negative
    );
}

#[cfg(feature = "typical")]
#[test]
fn typical_varints_support_values_larger_than_u128() {
    use number::typical::{Deserialize, Serialize};

    let number = "680564733841876926926749214863536422912/3"
        .parse::<Number>()
        .unwrap();
    let mut encoded = Vec::new();
    number.serialize(&mut encoded).unwrap();

    assert!(encoded.len() > 18);
    assert_eq!(
        Number::deserialize(std::io::Cursor::new(encoded)).unwrap(),
        number
    );
}

#[cfg(feature = "schemars")]
#[test]
fn schemars_schema_is_string() {
    let schema = schemars::schema_for!(Number);
    let schema = serde_json::to_value(schema).unwrap();

    assert_eq!(schema["title"], "Number");
    assert_eq!(schema["type"], "string");
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
        Pow::pow(Number::new_ratio_i64(2, 3), 3u8),
        Number::new_ratio_i64(8, 27)
    );
    assert_eq!(
        Pow::pow(Number::new_ratio_i64(2, 3), -2i8),
        Number::new_ratio_i64(9, 4)
    );
    assert_eq!(
        Pow::pow(&Number::new_ratio_i64(2, 3), -2i16),
        Number::new_ratio_i64(9, 4)
    );
    assert_eq!(
        Pow::pow(&Number::new_ratio_i64(2, 3), 3u8),
        Number::new_ratio_i64(8, 27)
    );
    assert_eq!(Number::one().signum() * 0, Number::zero());
    assert_eq!(Number::one().signum() * 1, Number::one());
    assert_eq!((-Number::one()).signum() * 0, Number::zero());
    assert_eq!((-Number::one()).signum() * 1, num!(-1));
}

#[cfg(feature = "num-rational")]
#[test]
fn creates_from_num_rational() {
    const RATIONAL: num_rational::Ratio<i64> = num_rational::Ratio::new_raw(-22, 7);
    const NUMBER: Number = Number::new_num_rational(RATIONAL);
    const I8: Number = Number::new_num_rational_i8(num_rational::Ratio::new_raw(-2, 3));
    const I16: Number = Number::new_num_rational_i16(num_rational::Ratio::new_raw(-4, 5));
    const I32: Number = Number::new_num_rational_i32(num_rational::Ratio::new_raw(-44, 14));
    const ISIZE: Number = Number::new_num_rational_isize(num_rational::Ratio::new_raw(-6, 7));
    const U8: Number = Number::new_num_rational_u8(num_rational::Ratio::new_raw(2, 3));
    const U16: Number = Number::new_num_rational_u16(num_rational::Ratio::new_raw(4, 5));
    const U32: Number = Number::new_num_rational_u32(num_rational::Ratio::new_raw(44, 14));
    const U64: Number = Number::new_num_rational_u64(num_rational::Ratio::new_raw(22, 7));
    const USIZE: Number = Number::new_num_rational_usize(num_rational::Ratio::new_raw(6, 7));

    assert_eq!(NUMBER, Number::new_ratio_i64(-22, 7));
    assert_eq!(I8, Number::new_ratio_i64(-2, 3));
    assert_eq!(I16, Number::new_ratio_i64(-4, 5));
    assert_eq!(I32, Number::new_ratio_i64(-44, 14));
    assert_eq!(ISIZE, Number::new_ratio_i64(-6, 7));
    assert_eq!(U8, Number::new_ratio_i64(2, 3));
    assert_eq!(U16, Number::new_ratio_i64(4, 5));
    assert_eq!(U32, Number::new_ratio_i64(44, 14));
    assert_eq!(U64, Number::new_ratio_i64(22, 7));
    assert_eq!(USIZE, Number::new_ratio_i64(6, 7));
    assert_eq!(
        Number::from(num_rational::Ratio::new(-44i32, 14)),
        Number::new_ratio_i64(-22, 7)
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

#[cfg(all(feature = "serde", feature = "float"))]
#[test]
fn serde_parses_decimal() {
    let json = "10.5";
    let num: Number = serde_json::from_str(json).unwrap();
    assert_eq!(num, Number::from(21) / 2);
}

#[cfg(feature = "serde")]
#[test]
fn serde_parses_decimal_string_without_float_feature() {
    let json = "\"10.5\"";
    let num: Number = serde_json::from_str(json).unwrap();
    assert_eq!(num, Number::from(21) / 2);
}

#[cfg(feature = "serde")]
#[test]
fn serde_parses_integer() {
    let json = "10";
    let num: Number = serde_json::from_str(json).unwrap();
    assert_eq!(num, Number::from(10i32));
}
