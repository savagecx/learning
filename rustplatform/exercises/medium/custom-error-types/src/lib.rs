#[derive(Debug, PartialEq, Eq)]
enum CalcError {
    DivisionByZero,
    ParseInt(String),
    BadFormat,
}

fn parse_int(tok: &str) -> Result<i32, CalcError> {
    tok.trim()
        .parse()
        .map_err(|_| CalcError::ParseInt(tok.to_string()))
}

fn safe_div(a: i32, b: i32) -> Result<i32, CalcError> {
    match b {
        0 => Err(CalcError::DivisionByZero),
        _ => Ok(a / b),
    }
}

fn eval_div(expr: &str) -> Result<i32, CalcError> {
    let parts: Vec<&str> = expr.split("/").collect();
    match parts {
        x if x.len() == 2 => safe_div(parse_int(x[0])?, parse_int(x[1])?),
        _ => Err(CalcError::BadFormat),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use CalcError::*;

    #[test]
    fn test_eval_ok() {
        assert_eq!(eval_div("12/3"), Ok(4));
        assert_eq!(eval_div(" 10 / 2 "), Ok(5));
        assert_eq!(eval_div("7/2"), Ok(3));
    }

    #[test]
    fn test_division_by_zero() {
        assert_eq!(eval_div("7/0"), Err(DivisionByZero));
    }

    #[test]
    fn test_parse_errors() {
        assert_eq!(eval_div("x/3"), Err(ParseInt("x".into())));
        assert_eq!(eval_div("9/y"), Err(ParseInt("y".into())));
    }

    #[test]
    fn test_bad_format() {
        assert_eq!(eval_div("1/2/3"), Err(BadFormat));
        assert_eq!(eval_div("no slash here"), Err(BadFormat));
        assert_eq!(eval_div(""), Err(BadFormat));
    }
}
