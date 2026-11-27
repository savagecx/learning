// Pattern 1: Return owned data
// Instead of complex lifetime juggling, just return an owned String
fn join_words(words: &[&str]) -> String {
    words.join(" ")
}

// Pattern 2: Split borrows
// When you need mutable access to different parts of a slice
fn swap_first_last(v: &mut [i32]) {
    if v.len() < 2 {
        return ();
    }
    v.swap(0, v.len() - 1);
}

// Pattern 3: Return owned to avoid lifetime issues
fn find_and_replace(text: &str, find: &str, replace: &str) -> String {
    text.replace(find, replace)
}

// Pattern 4: Process and return owned results
// When you need to both filter and aggregate
fn extract_and_process(data: &[i32]) -> (Vec<i32>, i32) {
    let evens = data
        .iter()
        .filter(|&&x| x % 2 == 0)
        .copied()
        .collect::<Vec<_>>();
    let sum_of_evens = evens.iter().sum();

    (evens, sum_of_evens)
}

// Pattern 5: Owned wrapper for complex operations
// When you need to build up data over time
struct Buffer {
    data: Vec<u8>,
}

impl Buffer {
    fn new() -> Self {
        Buffer { data: Vec::new() }
    }

    fn append(&mut self, bytes: &[u8]) {
        self.data.extend_from_slice(bytes);
    }

    fn get(&self, index: usize) -> Option<u8> {
        self.data.get(index).copied()
    }

    fn to_string_lossy(&self) -> String {
        String::from_utf8_lossy(&self.data).into_owned()
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_join_words_basic() {
        assert_eq!(join_words(&["hello", "world"]), "hello world");
    }

    #[test]
    fn test_join_words_single() {
        assert_eq!(join_words(&["hello"]), "hello");
    }

    #[test]
    fn test_join_words_empty() {
        let empty: &[&str] = &[];
        assert_eq!(join_words(empty), "");
    }

    #[test]
    fn test_join_words_many() {
        assert_eq!(join_words(&["a", "b", "c", "d"]), "a b c d");
    }

    #[test]
    fn test_swap_first_last_basic() {
        let mut nums = vec![1, 2, 3, 4, 5];
        swap_first_last(&mut nums);
        assert_eq!(nums, vec![5, 2, 3, 4, 1]);
    }

    #[test]
    fn test_swap_first_last_two() {
        let mut nums = vec![1, 2];
        swap_first_last(&mut nums);
        assert_eq!(nums, vec![2, 1]);
    }

    #[test]
    fn test_swap_first_last_one() {
        let mut nums = vec![1];
        swap_first_last(&mut nums);
        assert_eq!(nums, vec![1]);
    }

    #[test]
    fn test_swap_first_last_empty() {
        let mut nums: Vec<i32> = vec![];
        swap_first_last(&mut nums);
        assert!(nums.is_empty());
    }

    #[test]
    fn test_find_and_replace_basic() {
        assert_eq!(
            find_and_replace("hello world", "world", "rust"),
            "hello rust"
        );
    }

    #[test]
    fn test_find_and_replace_multiple() {
        assert_eq!(find_and_replace("a b a b a", "a", "x"), "x b x b x");
    }

    #[test]
    fn test_find_and_replace_not_found() {
        assert_eq!(find_and_replace("hello", "xyz", "abc"), "hello");
    }

    #[test]
    fn test_find_and_replace_empty() {
        assert_eq!(find_and_replace("", "a", "b"), "");
    }

    #[test]
    fn test_extract_and_process() {
        let (evens, sum) = extract_and_process(&[1, 2, 3, 4, 5, 6]);
        assert_eq!(evens, vec![2, 4, 6]);
        assert_eq!(sum, 12);
    }

    #[test]
    fn test_extract_and_process_no_evens() {
        let (evens, sum) = extract_and_process(&[1, 3, 5]);
        assert!(evens.is_empty());
        assert_eq!(sum, 0);
    }

    #[test]
    fn test_buffer_append_and_get() {
        let mut buf = Buffer::new();
        buf.append(b"hello");
        assert_eq!(buf.get(0), Some(b'h'));
        assert_eq!(buf.get(4), Some(b'o'));
        assert_eq!(buf.get(5), None);
    }

    #[test]
    fn test_buffer_to_string() {
        let mut buf = Buffer::new();
        buf.append(b"hello ");
        buf.append(b"world");
        assert_eq!(buf.to_string_lossy(), "hello world");
    }

    #[test]
    fn test_buffer_default() {
        let buf = Buffer::default();
        assert_eq!(buf.get(0), None);
    }
}
