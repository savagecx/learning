#[derive(Debug, PartialEq, Eq)]
enum JsonValue {
    Bool(bool),
    Null,
    Number(i64),
}

#[derive(Debug, PartialEq, Eq)]
enum ParseError {
    Empty,
    Invalid,
    Overflow,
}

fn parse_primitive(input: &str) -> Result<JsonValue, ParseError> {
    use std::num::IntErrorKind::*;
    match input.trim() {
        val if val.is_empty() => Err(ParseError::Empty),
        val if val == "true" || val == "false" => Ok(JsonValue::Bool(
            val.parse().expect("Match guard only allows bool values."),
        )),
        "null" => Ok(JsonValue::Null),
        val => Ok(JsonValue::Number(val.parse::<i64>().map_err(
            |err| match err.kind() {
                PosOverflow | NegOverflow => ParseError::Overflow,
                _ => ParseError::Invalid,
            },
        )?)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use JsonValue::*;
    use ParseError::*;

    #[test]
    fn test_bools_null() {
        assert_eq!(parse_primitive("true"), Ok(Bool(true)));
        assert_eq!(parse_primitive(" false "), Ok(Bool(false)));
        assert_eq!(parse_primitive("null"), Ok(Null));
    }

    #[test]
    fn test_integers() {
        assert_eq!(parse_primitive("42"), Ok(Number(42)));
        assert_eq!(parse_primitive("-7"), Ok(Number(-7)));
        assert_eq!(parse_primitive("+9"), Ok(Number(9)));
    }

    #[test]
    fn test_errors() {
        assert_eq!(parse_primitive(""), Err(Empty));
        assert_eq!(parse_primitive("  "), Err(Empty));
        assert_eq!(parse_primitive("12x"), Err(Invalid));
        assert_eq!(parse_primitive("+"), Err(Invalid));
        assert_eq!(parse_primitive("-"), Err(Invalid));
    }

    #[test]
    fn test_overflow() {
        assert_eq!(parse_primitive("99999999999999999999"), Err(Overflow));
        assert_eq!(parse_primitive("-99999999999999999999"), Err(Overflow));
    }
}
