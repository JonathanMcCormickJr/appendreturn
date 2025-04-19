//! Rust's `.push_str()` method for `String` does not return the resulting value. This crate fixes that by providing a new trait method for String. Just import this crate, then call `.append_and_return()` on a String like so. 
//! 
//! ```rust
//! use appendreturn::AppendReturn;
//! let original_value = String::from("Hello");
//! let additional_value = " world!!";
//! let combined_value = original_value.append_and_return(additional_value);
//! assert_eq!(combined_value, String::from("Hello world!!"));
//! ```


pub trait AppendReturn {
    fn append_and_return(self: Self, string: &str) -> Self;
}

impl AppendReturn for String {
    /// Calls the standard library's `push_str()` method and **returns** the resulting value.
    ///
    /// ```rust
    /// use appendreturn::AppendReturn;
    /// let original_value = String::from("Hello");
    /// let additional_value = " world!!";
    /// let combined_value = original_value.append_and_return(additional_value);
    /// assert_eq!(combined_value, String::from("Hello world!!"));
    /// ```
    fn append_and_return(self: Self, string: &str) -> Self {
        let mut return_value = self;
        return_value.push_str(string);
        return_value
    }
}
