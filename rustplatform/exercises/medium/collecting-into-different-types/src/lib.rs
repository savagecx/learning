use std::collections::HashMap;

fn group_by_first_char(words: &[&str]) -> HashMap<char, Vec<String>> {
    let mut grouped: HashMap<char, Vec<String>> = HashMap::new();
    for word in words {
        if word.is_empty() {
            continue;
        }
        grouped
            .entry(word.chars().next().unwrap())
            .or_default()
            .push(word.to_string());
    }

    grouped
}

fn char_frequencies(s: &str) -> HashMap<char, usize> {
    let mut frequencies = HashMap::new();
    for ch in s.chars() {
        *frequencies.entry(ch).or_default() += 1;
    }
    frequencies
}

fn invert_map<K, V>(map: HashMap<K, V>) -> HashMap<V, K>
where
    K: std::hash::Hash + Eq,
    V: std::hash::Hash + Eq,
{
    map.into_iter().map(|(k, v)| (v, k)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_by_first_char_basic() {
        let groups = group_by_first_char(&["apple", "avocado", "banana", "blueberry"]);
        let mut a = groups.get(&'a').unwrap().clone();
        a.sort();
        assert_eq!(a, vec!["apple", "avocado"]);
        let mut b = groups.get(&'b').unwrap().clone();
        b.sort();
        assert_eq!(b, vec!["banana", "blueberry"]);
    }

    #[test]
    fn test_group_by_first_char_single_group() {
        let groups = group_by_first_char(&["cat", "car", "cup"]);
        assert_eq!(groups.len(), 1);
        assert_eq!(groups.get(&'c').unwrap().len(), 3);
    }

    #[test]
    fn test_group_by_first_char_empty() {
        let groups = group_by_first_char(&[]);
        assert!(groups.is_empty());
    }

    #[test]
    fn test_group_by_first_char_single_word() {
        let groups = group_by_first_char(&["hello"]);
        assert_eq!(groups.get(&'h').unwrap(), &vec!["hello".to_string()]);
    }

    #[test]
    fn test_group_by_first_char_skips_empty_strings() {
        let groups = group_by_first_char(&["", "apple", ""]);
        assert_eq!(groups.len(), 1);
        assert_eq!(groups.get(&'a').unwrap(), &vec!["apple".to_string()]);
    }

    #[test]
    fn test_char_frequencies_basic() {
        let freq = char_frequencies("hello");
        assert_eq!(freq.get(&'h'), Some(&1));
        assert_eq!(freq.get(&'e'), Some(&1));
        assert_eq!(freq.get(&'l'), Some(&2));
        assert_eq!(freq.get(&'o'), Some(&1));
    }

    #[test]
    fn test_char_frequencies_empty() {
        let freq = char_frequencies("");
        assert!(freq.is_empty());
    }

    #[test]
    fn test_char_frequencies_single_char() {
        let freq = char_frequencies("aaa");
        assert_eq!(freq.get(&'a'), Some(&3));
        assert_eq!(freq.len(), 1);
    }

    #[test]
    fn test_char_frequencies_with_spaces() {
        let freq = char_frequencies("a b c");
        assert_eq!(freq.get(&' '), Some(&2));
    }

    #[test]
    fn test_invert_map_basic() {
        let map: HashMap<&str, i32> = [("a", 1), ("b", 2)].into_iter().collect();
        let inverted = invert_map(map);
        assert_eq!(inverted.get(&1), Some(&"a"));
        assert_eq!(inverted.get(&2), Some(&"b"));
    }

    #[test]
    fn test_invert_map_empty() {
        let map: HashMap<&str, i32> = HashMap::new();
        let inverted = invert_map(map);
        assert!(inverted.is_empty());
    }

    #[test]
    fn test_invert_map_single() {
        let map: HashMap<String, i32> = [(String::from("x"), 42)].into_iter().collect();
        let inverted = invert_map(map);
        assert_eq!(inverted.get(&42), Some(&String::from("x")));
    }

    #[test]
    fn test_invert_map_numbers() {
        let map: HashMap<i32, i32> = [(1, 10), (2, 20), (3, 30)].into_iter().collect();
        let inverted = invert_map(map);
        assert_eq!(inverted.get(&10), Some(&1));
        assert_eq!(inverted.get(&20), Some(&2));
        assert_eq!(inverted.get(&30), Some(&3));
    }
}
