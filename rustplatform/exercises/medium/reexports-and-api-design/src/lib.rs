mod internal {
    pub mod types {
        use crate::internal::errors::ValidationError;

        pub struct User {
            pub name: String,
            pub age: u32,
        }

        impl User {
            pub fn new(name: &str, age: u32) -> Result<User, ValidationError> {
                if name.is_empty() || age > 150 {
                    return Err(ValidationError("name cannot be empty".to_string()));
                } else if age > 150 {
                    return Err(ValidationError("age must be less than 150".to_string()));
                }

                Ok(Self {
                    name: name.to_string(),
                    age,
                })
            }
        }
    }

    pub mod errors {
        #[derive(Debug)]
        pub struct ValidationError(pub String);
    }
}

pub mod prelude {
    pub use crate::internal::errors::ValidationError;
    pub use crate::internal::types::User;
}

pub use crate::internal::types::User;

#[cfg(test)]
mod tests {
    use super::prelude::*;

    #[test]
    fn test_user_new_valid() {
        let user = User::new("Alice", 30).unwrap();
        assert_eq!(user.name, "Alice");
        assert_eq!(user.age, 30);
    }

    #[test]
    fn test_user_new_empty_name() {
        let result = User::new("", 30);
        assert!(result.is_err());
    }

    #[test]
    fn test_user_new_age_too_high() {
        let result = User::new("Alice", 200);
        assert!(result.is_err());
    }

    #[test]
    fn test_user_new_max_age() {
        let user = User::new("Elder", 150).unwrap();
        assert_eq!(user.age, 150);
    }

    #[test]
    fn test_direct_import() {
        let user = crate::User::new("Bob", 25).unwrap();
        assert_eq!(user.name, "Bob");
    }

    #[test]
    fn test_prelude_import() {
        use crate::prelude::*;
        let user = User::new("Carol", 40).unwrap();
        let _ = ValidationError("test".to_string());
        assert_eq!(user.name, "Carol");
    }
}
