use crate::Number;

struct NumberVisitor;

impl<'de> ::serde::de::Visitor<'de> for NumberVisitor {
    type Value = Number;

    fn expecting(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        formatter.write_str("a rational string or finite JSON number")
    }

    fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E>
    where
        E: ::serde::de::Error,
    {
        value.parse().map_err(|()| E::custom("invalid rational"))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: ::serde::de::Error,
    {
        value.parse().map_err(|()| E::custom("invalid rational"))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: ::serde::de::Error,
    {
        Ok(Number::new_i64(value))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: ::serde::de::Error,
    {
        Ok(Number::new_u64(value))
    }

    #[cfg(feature = "float")]
    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: ::serde::de::Error,
    {
        Number::try_new_f64(value).map_err(|_| E::custom("invalid finite float"))
    }
}

impl ::serde::Serialize for Number {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> ::serde::Deserialize<'de> for Number {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(NumberVisitor)
    }
}
