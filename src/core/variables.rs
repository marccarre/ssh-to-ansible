use derive_more::Display;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use thiserror::Error;

lazy_static! {
    static ref QUOTED_INTEGER: Regex = Regex::new(r#"^['\\""]*(?<integer>\d+)['\\""]*$"#).unwrap();
}

#[derive(Clone, Debug, Deserialize, Display, PartialEq, Serialize)]
pub enum ValueType {
    #[display("{_0}")]
    #[serde(untagged)]
    Bool(bool),
    #[display("{_0}")]
    #[serde(untagged)]
    Int64(i64),
    #[display("{_0}")]
    #[serde(untagged)]
    Float64(f64),
    #[display("{_0}")]
    #[serde(untagged)]
    String(String),
}

#[derive(Debug, Display, Error, PartialEq, Eq)]
pub struct ParseValueTypeError;

impl FromStr for ValueType {
    // ParseValueTypeError is never used in practice, as we default to `ValueType::String`.
    type Err = ParseValueTypeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if let Ok(parsed_value) = bool::from_str(value) {
            return Ok(ValueType::Bool(parsed_value));
        }
        if let Ok(parsed_value) = i64::from_str(value) {
            return Ok(ValueType::Int64(parsed_value));
        }
        if let Ok(parsed_value) = f64::from_str(value) {
            return Ok(ValueType::Float64(parsed_value));
        }
        if let Some(captures) = QUOTED_INTEGER.captures(value) {
            if let Some(integer) = captures.name("integer") {
                return Ok(ValueType::String(integer.as_str().to_string()));
            }
        }
        Ok(ValueType::String(value.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use crate::core::variables::ValueType;
    use rstest::rstest;
    use serde_yaml;
    use std::str::FromStr;

    use super::ParseValueTypeError;

    #[rstest]
    // Given:
    #[case::parse_true("true", ValueType::Bool(true))]
    #[case::parse_false("false", ValueType::Bool(false))]
    #[case::parse_zero("0", ValueType::Int64(0))]
    #[case::parse_one("1", ValueType::Int64(1))]
    #[case::parse_minus_one("-1", ValueType::Int64(-1))]
    #[case::parse_pi("1.23", ValueType::Float64(1.23))]
    #[case::parse_unit("3G", ValueType::String("3G".to_string()))]
    #[case::parse_user("root", ValueType::String("root".to_string()))]
    #[case::parse_port_as_single_quoted_str("'22'", ValueType::String("22".to_string()))]
    #[case::parse_port_as_double_quoted_str(r#""22""#, ValueType::String("22".to_string()))]
    fn from_str(
        #[case] input: &str,
        #[case] expected: ValueType,
    ) -> Result<(), ParseValueTypeError> {
        // When:
        let value = ValueType::from_str(input)?;
        // Then:
        assert_eq!(value, expected);
        Ok(())
    }

    #[rstest]
    // Given:
    #[case::serialize_bool(ValueType::Bool(true), "true")]
    #[case::serialize_int(ValueType::Int64(1337), "1337")]
    #[case::serialize_float(ValueType::Float64(1.23), "1.23")]
    #[case::serialize_str(ValueType::String("foo".to_string()), "foo")]
    #[case::serialize_int_as_str(ValueType::String("22".to_string()), "'22'")]
    fn serialize_to_yaml(
        #[case] input: ValueType,
        #[case] expected: &str,
    ) -> Result<(), serde_yaml::Error> {
        // When:
        let yaml = serde_yaml::to_string(&input)?;
        // Then:
        assert_eq!(yaml.trim(), expected);
        Ok(())
    }
}
