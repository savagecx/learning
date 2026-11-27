fn translate(text: &str, from: char, to: char) -> String {
    text.chars()
        .map(|ch| if ch == from { to } else { ch })
        .collect()
}

fn delete(text: &str, ch: char) -> String {
    text.chars().filter(|&x| x != ch).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate_replaces_every_match() {
        assert_eq!(translate("hello", 'l', 'L'), "heLLo");
    }

    #[test]
    fn test_translate_no_match() {
        assert_eq!(translate("abc", 'z', 'Z'), "abc");
    }

    #[test]
    fn test_delete_removes_every_match() {
        assert_eq!(delete("h.e.l.l.o", '.'), "hello");
    }

    #[test]
    fn test_delete_no_match() {
        assert_eq!(delete("abc", 'z'), "abc");
    }

    #[test]
    fn test_empty_input() {
        assert_eq!(translate("", 'a', 'b'), "");
        assert_eq!(delete("", 'a'), "");
    }
}
