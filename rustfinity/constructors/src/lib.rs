pub struct Book {
    pub title: String,
    pub author: String,
    pub year: i16,
    pub likes: u32,
}

impl Book {
    pub fn new(title: &str, author: &str, year: i16) -> Book {
        Book {
            title: String::from(title),
            author: String::from(author),
            year,
            likes: 0,
        }
    }
}
