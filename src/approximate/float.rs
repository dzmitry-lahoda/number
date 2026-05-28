use crate::Number;

impl Number {
    pub fn try_new_f32(
        value: f32,
    ) -> Result<Self, malachite_q::conversion::from_primitive_float::RationalFromPrimitiveFloatError>
    {
        malachite_q::Rational::try_from(value).map(Self)
    }

    pub fn try_new_f64(
        value: f64,
    ) -> Result<Self, malachite_q::conversion::from_primitive_float::RationalFromPrimitiveFloatError>
    {
        malachite_q::Rational::try_from(value).map(Self)
    }

    pub fn new_f32(value: f32) -> Self {
        Self::try_new_f32(value).expect("float value should be finite")
    }

    pub fn new_f64(value: f64) -> Self {
        Self::try_new_f64(value).expect("float value should be finite")
    }
}

impl From<f32> for Number {
    fn from(value: f32) -> Self {
        Self::new_f32(value)
    }
}

impl From<f64> for Number {
    fn from(value: f64) -> Self {
        Self::new_f64(value)
    }
}
