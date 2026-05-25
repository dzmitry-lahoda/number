use crate::Number;

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
        let value = <::std::borrow::Cow<'de, str>>::deserialize(deserializer)?;
        value
            .parse()
            .map(Self)
            .map_err(|()| ::serde::de::Error::custom("invalid rational"))
    }
}
