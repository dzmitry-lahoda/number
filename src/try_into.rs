use crate::Number;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TryFromNumberError;

macro_rules! impl_try_from_primitive {
    ($type:ty) => {
        impl TryFrom<&Number> for $type {
            type Error = TryFromNumberError;

            fn try_from(value: &Number) -> Result<Self, Self::Error> {
                <$type as TryFrom<&malachite_q::Rational>>::try_from(&value.0)
                    .map_err(|_| TryFromNumberError)
            }
        }

        impl TryFrom<Number> for $type {
            type Error = TryFromNumberError;

            fn try_from(value: Number) -> Result<Self, Self::Error> {
                <$type as TryFrom<&Number>>::try_from(&value)
            }
        }
    };
}

macro_rules! impl_try_from_nonzero {
    ($type:ty, $inner:ty) => {
        impl TryFrom<&Number> for $type {
            type Error = TryFromNumberError;

            fn try_from(value: &Number) -> Result<Self, Self::Error> {
                let value = <$inner as TryFrom<&Number>>::try_from(value)?;
                <$type>::new(value).ok_or(TryFromNumberError)
            }
        }

        impl TryFrom<Number> for $type {
            type Error = TryFromNumberError;

            fn try_from(value: Number) -> Result<Self, Self::Error> {
                <$type as TryFrom<&Number>>::try_from(&value)
            }
        }
    };
}

impl_try_from_primitive!(i8);
impl_try_from_primitive!(i16);
impl_try_from_primitive!(i32);
impl_try_from_primitive!(i64);
impl_try_from_primitive!(i128);
impl_try_from_primitive!(isize);
impl_try_from_primitive!(u8);
impl_try_from_primitive!(u16);
impl_try_from_primitive!(u32);
impl_try_from_primitive!(u64);
impl_try_from_primitive!(u128);
impl_try_from_primitive!(usize);

impl_try_from_nonzero!(core::num::NonZeroI8, i8);
impl_try_from_nonzero!(core::num::NonZeroI16, i16);
impl_try_from_nonzero!(core::num::NonZeroI32, i32);
impl_try_from_nonzero!(core::num::NonZeroI64, i64);
impl_try_from_nonzero!(core::num::NonZeroI128, i128);
impl_try_from_nonzero!(core::num::NonZeroIsize, isize);
impl_try_from_nonzero!(core::num::NonZeroU8, u8);
impl_try_from_nonzero!(core::num::NonZeroU16, u16);
impl_try_from_nonzero!(core::num::NonZeroU32, u32);
impl_try_from_nonzero!(core::num::NonZeroU64, u64);
impl_try_from_nonzero!(core::num::NonZeroU128, u128);
impl_try_from_nonzero!(core::num::NonZeroUsize, usize);
