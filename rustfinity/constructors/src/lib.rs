pub struct Book {
    pub title: String,
    pub author: String,
    pub year: i64,
    pub likes: i64,
}

impl Book {
    pub fn new(title: &str, author: &str, year: i64) -> Book {
        Book {
            title: title.to_string(),
            author: author.to_string(),
            year,
            likes: 0,
        }
    }
}
