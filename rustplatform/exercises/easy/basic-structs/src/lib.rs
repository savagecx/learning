struct Book {
    title: String,
    author: String,
    pages: u32,
}

impl Book {
    fn describe(&self) -> String {
        format!(
            "The book '{}' by {} has {} pages.",
            self.title, self.author, self.pages
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_describe_book() {
        let book = Book {
            title: "The Hobbit".to_string(),
            author: "J.R.R. Tolkien".to_string(),
            pages: 310,
        };
        assert_eq!(
            book.describe(),
            "The book 'The Hobbit' by J.R.R. Tolkien has 310 pages."
        );
    }

    #[test]
    fn test_describe_war_and_peace() {
        let book = Book {
            title: "War and Peace".to_string(),
            author: "Leo Tolstoy".to_string(),
            pages: 1225,
        };
        assert_eq!(
            book.describe(),
            "The book 'War and Peace' by Leo Tolstoy has 1225 pages."
        );
    }
}
