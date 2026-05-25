use crate::Number;

macro_rules! impl_from_nonzero {
    ($type:ty, $constructor:ident) => {
        impl From<$type> for Number {
            fn from(value: $type) -> Self {
                Self::$constructor(value)
            }
        }
    };
}

macro_rules! impl_from_primitive {
    ($type:ty, $constructor:ident) => {
        impl From<$type> for Number {
            fn from(value: $type) -> Self {
                Self::$constructor(value)
            }
        }
    };
}

impl From<&Number> for Number {
    fn from(value: &Number) -> Self {
        value.clone()
    }
}

impl From<bool> for Number {
    fn from(value: bool) -> Self {
        if value {
            Self::new_i64(1)
        } else {
            Self::new_i64(-1)
        }
    }
}

impl From<Option<bool>> for Number {
    fn from(value: Option<bool>) -> Self {
        match value {
            Some(value) => value.into(),
            None => Self::new_i64(0),
        }
    }
}

impl_from_primitive!(i8, new_i8);
impl_from_primitive!(i16, new_i16);
impl_from_primitive!(i32, new_i32);
impl_from_primitive!(i64, new_i64);
impl_from_primitive!(i128, new_i128);
impl_from_primitive!(isize, new_isize);
impl_from_primitive!(u8, new_u8);
impl_from_primitive!(u16, new_u16);
impl_from_primitive!(u32, new_u32);
impl_from_primitive!(u64, new_u64);
impl_from_primitive!(u128, new_u128);
impl_from_primitive!(usize, new_usize);

impl_from_nonzero!(core::num::NonZeroI8, new_nonzero_i8);
impl_from_nonzero!(core::num::NonZeroI16, new_nonzero_i16);
impl_from_nonzero!(core::num::NonZeroI32, new_nonzero_i32);
impl_from_nonzero!(core::num::NonZeroI64, new_nonzero_i64);
impl_from_nonzero!(core::num::NonZeroI128, new_nonzero_i128);
impl_from_nonzero!(core::num::NonZeroIsize, new_nonzero_isize);
impl_from_nonzero!(core::num::NonZeroU8, new_nonzero_u8);
impl_from_nonzero!(core::num::NonZeroU16, new_nonzero_u16);
impl_from_nonzero!(core::num::NonZeroU32, new_nonzero_u32);
impl_from_nonzero!(core::num::NonZeroU64, new_nonzero_u64);
impl_from_nonzero!(core::num::NonZeroU128, new_nonzero_u128);
impl_from_nonzero!(core::num::NonZeroUsize, new_nonzero_usize);
