pub trait Describable {
    fn describe(&self) -> String;
}

pub struct Person {
    pub name: String,
    pub age: u8,
}

impl Describable for Person {
    fn describe(&self) -> String {
        format!("Person: {}, Age: {}", self.name, self.age)
    }
}

pub struct Book {
    pub title: String,
    pub author: String,
}

impl Describable for Book {
    fn describe(&self) -> String {
        format!("Book: {}, Author: {}", self.title, self.author)
    }
}

// Example usage
pub fn main() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };

    let book = Book {
        title: "Rust Programming".to_string(),
        author: "Jane Doe".to_string(),
    };

    println!("{}", person.describe());
    println!("{}", book.describe());
}
