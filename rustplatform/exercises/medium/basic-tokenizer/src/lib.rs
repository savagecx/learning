#[derive(Debug, PartialEq, Eq)]
enum Token {
    Ident(String),
    Number(i64),
    LParen,
    RParen,
    Plus,
    Minus,
    Star,
    Slash,
    Comma,
    Equal,
    Unknown(char),
}

/* Helpers: ASCII only */
fn is_ident_start(c: char) -> bool {
    // TODO: return true for letter or '_'
    todo!()
}
fn is_ident_continue(c: char) -> bool {
    // TODO: return true for letter/digit or '_'
    todo!()
}

fn tokenize(input: &str) -> Vec<Token> {
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    let n = chars.len();
    let mut out = Vec::new();

    while i < n {
        // 1) skip whitespace (hint: is_whitespace())
        //    - advance i past any whitespace, then continue

        // 2) identifiers: first char letter/_ ; rest letter/digit/_
        //    - let start = i; advance i while is_ident_continue(chars[i])
        //    - slice chars[start..i], collect into String
        //    - out.push(Token::Ident(...)); continue;

        // 3) numbers: one or more digits (hint: is_ascii_digit())
        //    - similar pattern: start, advance, slice, parse::<i64>().unwrap_or(0)
        //    - out.push(Token::Number(...)); continue;

        // 4) single-char tokens and unknown fallback
        //    - use a `match chars[i] { ... }` to map: ( ) + - * / = ,
        //    - push the right Token, unknown chars go to Token::Unknown
        //    - remember to advance i after handling

        todo!()
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use Token::*;

    #[test]
    fn test_basic() {
        let toks = tokenize("sum(x, 42) - y3");
        assert_eq!(
            toks,
            vec![
                Ident("sum".into()),
                LParen,
                Ident("x".into()),
                Comma,
                Number(42),
                RParen,
                Minus,
                Ident("y3".into())
            ]
        );
    }

    #[test]
    fn test_empty_and_ws() {
        assert_eq!(tokenize(""), vec![]);
        assert_eq!(tokenize("   \t\n"), vec![]);
    }

    #[test]
    fn test_unknown() {
        let toks = tokenize("@");
        assert_eq!(toks, vec![Unknown('@')]);
    }

    #[test]
    fn test_long_ident_and_number() {
        let toks = tokenize("foo_bar123 + 9000");
        assert_eq!(toks, vec![Ident("foo_bar123".into()), Plus, Number(9000)]);
    }
}