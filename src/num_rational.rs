use crate::Number;

impl Number {
    pub const fn new_num_rational(value: num_rational::Ratio<i64>) -> Self {
        Self::new_ratio_i64(*value.numer(), *value.denom())
    }

    pub const fn new_num_rational_i8(value: num_rational::Ratio<i8>) -> Self {
        Self::new_ratio_i64(*value.numer() as i64, *value.denom() as i64)
    }

    pub const fn new_num_rational_i16(value: num_rational::Ratio<i16>) -> Self {
        Self::new_ratio_i64(*value.numer() as i64, *value.denom() as i64)
    }

    pub const fn new_num_rational_i32(value: num_rational::Ratio<i32>) -> Self {
        Self::new_ratio_i64(*value.numer() as i64, *value.denom() as i64)
    }

    pub const fn new_num_rational_isize(value: num_rational::Ratio<isize>) -> Self {
        Self::new_ratio_i64(*value.numer() as i64, *value.denom() as i64)
    }

    pub const fn new_num_rational_u8(value: num_rational::Ratio<u8>) -> Self {
        Self::__from_unsigned_ratio_parts(*value.numer() as u64, *value.denom() as u64, false)
    }

    pub const fn new_num_rational_u16(value: num_rational::Ratio<u16>) -> Self {
        Self::__from_unsigned_ratio_parts(*value.numer() as u64, *value.denom() as u64, false)
    }

    pub const fn new_num_rational_u32(value: num_rational::Ratio<u32>) -> Self {
        Self::__from_unsigned_ratio_parts(*value.numer() as u64, *value.denom() as u64, false)
    }

    pub const fn new_num_rational_u64(value: num_rational::Ratio<u64>) -> Self {
        Self::__from_unsigned_ratio_parts(*value.numer(), *value.denom(), false)
    }

    pub const fn new_num_rational_usize(value: num_rational::Ratio<usize>) -> Self {
        Self::__from_unsigned_ratio_parts(*value.numer() as u64, *value.denom() as u64, false)
    }
}

macro_rules! impl_from_num_rational {
    ($type:ty, $constructor:ident) => {
        impl From<num_rational::Ratio<$type>> for Number {
            fn from(value: num_rational::Ratio<$type>) -> Self {
                Self::$constructor(value)
            }
        }
    };
}

impl_from_num_rational!(i8, new_num_rational_i8);
impl_from_num_rational!(i16, new_num_rational_i16);
impl_from_num_rational!(i32, new_num_rational_i32);
impl_from_num_rational!(i64, new_num_rational);
impl_from_num_rational!(isize, new_num_rational_isize);
impl_from_num_rational!(u8, new_num_rational_u8);
impl_from_num_rational!(u16, new_num_rational_u16);
impl_from_num_rational!(u32, new_num_rational_u32);
impl_from_num_rational!(u64, new_num_rational_u64);
impl_from_num_rational!(usize, new_num_rational_usize);
