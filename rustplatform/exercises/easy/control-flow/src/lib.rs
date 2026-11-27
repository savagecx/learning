fn grade_message(score: i32) -> String {
    // TODO:
    // 1. Use if / else if / else to get "Excellent", "Good", "Pass", or "Fail"
    // 2. Use match on the first letter to return:
    //    "Top", "Decent", "Basic", or "None"
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top() {
        assert_eq!(grade_message(95), "Top");
    }

    #[test]
    fn test_decent() {
        assert_eq!(grade_message(78), "Decent");
    }

    #[test]
    fn test_basic() {
        assert_eq!(grade_message(52), "Basic");
    }

    #[test]
    fn test_none() {
        assert_eq!(grade_message(10), "None");
    }
}
