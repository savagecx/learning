// --- Ownership: move in ---
fn transfer_ownership(name: String) -> String {
    format!("Welcome, {name}!")
}

// --- Borrowing: shared ---
fn longest_word_length(word1: &String, word2: &String) -> usize {
    word1.len().max(word2.len())
}

// --- Borrowing: mutable ---
fn to_uppercase(text: &mut String) {
    text.make_ascii_uppercase();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transfer_ownership() {
        let username = String::from("Alice");
        let result = transfer_ownership(username);
        assert_eq!(result, "Welcome, Alice!");
    }

    #[test]
    fn test_longest_word_length() {
        let text1 = String::from("borrow");
        let text2 = String::from("rust");
        let result = longest_word_length(&text1, &text2);
        assert_eq!(result, 6);

        let text3 = String::from("longerword");
        let result = longest_word_length(&text1, &text3);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_longest_word_length_same_length() {
        let text1 = String::from("hello");
        let text2 = String::from("world");
        assert_eq!(longest_word_length(&text1, &text2), 5);
    }

    #[test]
    fn test_to_uppercase() {
        let mut word = String::from("rust");
        to_uppercase(&mut word);
        assert_eq!(word, "RUST");
    }

    #[test]
    fn test_to_uppercase_with_numbers() {
        let mut word = String::from("rust2024");
        to_uppercase(&mut word);
        assert_eq!(word, "RUST2024");
    }

    #[test]
    fn test_to_uppercase_empty_string() {
        let mut word = String::from("");
        to_uppercase(&mut word);
        assert_eq!(word, "");
    }
}
