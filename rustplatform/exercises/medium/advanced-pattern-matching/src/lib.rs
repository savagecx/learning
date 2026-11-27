#[derive(Debug, PartialEq, Eq)]
enum Message {
    Ping,
    Echo(String),
    Move { x: i32, y: i32 },
    Quit,
}

fn classify_char(c: char) -> &'static str {
    match c {
        '0'..='9' => "digit",
        'a'..='z' | 'A'..='Z' | '_' => "alpha",
        '+' | '-' | '*' | '/' | '=' => "op",
        ' ' | '\t' | '\n' => "ws",
        _ => "other",
    }
}

fn sum_positive_pairs(input: &[(i32, i32)]) -> i32 {
    let mut total = 0;
    for &pair in input {
        total += match pair {
            (x, y) if x > 0 && y > 0 => x + y,
            _ => 0,
        }
    }
    total
}

fn route(msg: Message) -> String {
    match msg {
        Message::Ping => "pong".to_string(),
        Message::Echo(s) if s.is_empty() => "silence".to_string(),
        Message::Echo(s) => s.to_string(),
        Message::Move { x, y } if x == 0 && y == 0 => "noop".to_string(),
        Message::Move { x, y } => format!("move:{x},{y}"),
        Message::Quit => "bye".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Message::*;

    #[test]
    fn test_classify_char() {
        assert_eq!(classify_char('8'), "digit");
        assert_eq!(classify_char('_'), "alpha");
        assert_eq!(classify_char('+'), "op");
        assert_eq!(classify_char('\n'), "ws");
        assert_eq!(classify_char('?'), "other");
    }

    #[test]
    fn test_sum_positive_pairs() {
        assert_eq!(sum_positive_pairs(&[]), 0);
        assert_eq!(sum_positive_pairs(&[(1, 2), (-1, 3), (4, 0)]), 3);
        assert_eq!(sum_positive_pairs(&[(2, 2), (3, 3)]), 10);
    }

    #[test]
    fn test_route() {
        assert_eq!(route(Ping), "pong");
        assert_eq!(route(Echo("hi".into())), "hi");
        assert_eq!(route(Echo("".into())), "silence");
        assert_eq!(route(Move { x: 0, y: 0 }), "noop");
        assert_eq!(route(Move { x: 2, y: -1 }), "move:2,-1");
        assert_eq!(route(Quit), "bye");
    }
}
