use std::borrow::Cow;

use crate::Number;

impl ::schemars::JsonSchema for Number {
    fn schema_name() -> Cow<'static, str> {
        "Number".into()
    }

    fn schema_id() -> Cow<'static, str> {
        "number::Number".into()
    }

    fn json_schema(_generator: &mut ::schemars::SchemaGenerator) -> ::schemars::Schema {
        ::schemars::json_schema!({
            "type": "string",
            "description": "Exact rational number encoded as a decimal integer or numerator/denominator string."
        })
    }
}
