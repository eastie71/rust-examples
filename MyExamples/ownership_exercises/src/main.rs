//! Rust Ownership, Borrowing, and Moving Exercises
//! 
//! Instructions: 
//! - Solve each function to make the corresponding test pass
//! - Run tests using `cargo test`
//! - Focus on understanding ownership rules and Rust's borrowing mechanics

/// Exercise 1 (Easy): Basic Ownership
/// Create a function that takes ownership of a String and returns its length
fn calculate_string_length(s: String) -> usize {
    // Implement this function to return the length of the string
    s.len()
}

/// Exercise 2 (Easy): Borrowing
/// Create a function that borrows a string and returns its first character
fn get_first_char(s: &String) -> Option<char> {
    // Return the first character of the string if it exists
    if s.len() > 0 {
        let first_char = s.chars().next();
        first_char
    } else {
        None
    }
}

/// Exercise 3 (Medium): Ownership Transfer
/// Create a function that takes a vector, doubles each element, and returns the modified vector
fn double_vector(mut vec: Vec<i32>) -> Vec<i32> {
    // Takes ownership of the vector - returns a modified version
    // Modify each element by multiplying by 2 and return the vector
    for i in 0..vec.len() {
        vec[i] *= 2;
    }
    vec
}

/// Exercise 4 (Medium): Borrowing and Mutability
/// Create a function that takes a mutable reference to a vector and sorts it in-place
fn sort_vector_in_place(vec: &mut Vec<i32>) {
    // Sort the vector in ascending order
   vec.sort()
}

/// Exercise 5 (Hard): Lifetime and Borrowing
/// Create a function that finds the longest string in a slice of strings
fn find_longest_string<'a>(strings: &'a [String]) -> Option<&'a String> {
    // TODO: Return a reference to the longest string in the slice
    unimplemented!()
}

/// Exercise 6 (Hard): Complex Ownership and Cloning
/// Create a struct that manages a collection of strings with custom memory management
#[derive(Clone)]
struct StringManager {
    strings: Vec<String>,
}

impl StringManager {
    /// Create a new StringManager
    fn new() -> Self {
        // TODO: Implement a constructor that initializes an empty vector
        unimplemented!()
    }

    /// Add a new string to the manager
    fn add_string(&mut self, s: String) {
        // TODO: Add the string to the internal vector
        unimplemented!()
    }

    /// Get a reference to a string by index
    fn get_string(&self, index: usize) -> Option<&String> {
        // TODO: Return a reference to the string at the given index
        unimplemented!()
    }
}

/// Test module to verify solutions
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_string_length() {
        let s = String::from("hello");
        assert_eq!(calculate_string_length(s), 5);
        // Note: s is no longer valid after the function call due to ownership transfer
    }

    #[test]
    fn test_get_first_char() {
        let s = String::from("rust");
        assert_eq!(get_first_char(&s), Some('r'));
        assert_eq!(get_first_char(&String::new()), None);
        assert_eq!(s.as_str(), "rust")
    }

    #[test]
    fn test_double_vector() {
        let vec = vec![1, 2, 3, 4, 5];
        let doubled = double_vector(vec);
        assert_eq!(doubled, vec![2, 4, 6, 8, 10]);
        // next line fails compile, due to vec losing ownership
        //assert_eq!(vec, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_sort_vector_in_place() {
        let mut vec = vec![5, 2, 8, 1, 9];
        sort_vector_in_place(&mut vec);
        assert_eq!(vec, vec![1, 2, 5, 8, 9]);
    }

    #[test]
    fn test_find_longest_string() {
        let strings = vec![
            String::from("short"),
            String::from("longer"),
            String::from("longest string")
        ];
        let longest = find_longest_string(&strings);
        assert_eq!(longest, Some(&strings[2]));
    }

    #[test]
    fn test_string_manager() {
        let mut manager = StringManager::new();
        manager.add_string(String::from("hello"));
        manager.add_string(String::from("world"));

        assert_eq!(manager.get_string(0), Some(&String::from("hello")));
        assert_eq!(manager.get_string(1), Some(&String::from("world")));
        assert_eq!(manager.get_string(2), None);

        // Test cloning
        let cloned_manager = manager.clone();
        assert_eq!(cloned_manager.get_string(0), Some(&String::from("hello")));
    }
}

// To run these tests, create a new Rust project with Cargo:
// 1. `cargo new rust_ownership_exercises`
// 2. Replace src/main.rs with this entire file
// 3. Run `cargo test`