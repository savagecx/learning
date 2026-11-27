#[derive(Debug, PartialEq)]
enum CutError {
    ZeroField,
}

fn cut(text: &str, delim: char, field: usize) -> Result<Vec<&str>, CutError> {
    // if field == 0 {
    //     return Err(CutError::ZeroField);
    // }
    let index = field.checked_sub(1).ok_or(CutError::ZeroField)?;

    Ok(text
        .lines()
        .filter_map(|line| line.split(delim).nth(index))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_field() {
        assert_eq!(
            cut("root:x:0\ndaemon:x:1", ':', 1),
            Ok(vec!["root", "daemon"])
        );
    }

    #[test]
    fn test_later_field() {
        assert_eq!(cut("root:x:0\ndaemon:x:1", ':', 3), Ok(vec!["0", "1"]));
    }

    #[test]
    fn test_missing_field_skipped() {
        assert_eq!(cut("root:x:0\ndaemon:x:1", ':', 5), Ok(Vec::<&str>::new()));
    }

    #[test]
    fn test_space_delimiter() {
        assert_eq!(cut("a b c\nd e", ' ', 2), Ok(vec!["b", "e"]));
    }

    #[test]
    fn test_field_zero_is_error() {
        assert_eq!(cut("a:b", ':', 0), Err(CutError::ZeroField));
    }
}
