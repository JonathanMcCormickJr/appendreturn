pub trait AppendReturn {
    fn append_and_return(self: Self, string: &str) -> Self;
}

impl AppendReturn for String {
    /// Calls the standard library's `push_str()` method and **returns** the resulting value.
    ///
    /// ```rust
    /// use appendreturn::AppendReturn;
    ///
    /// fn test_append_return_for_string() {
    ///     let original_value = String::from("Hello");
    ///     let additional_value = " world!!";
    ///     let combined_value = original_value.append_and_return(additional_value);
    ///     assert_eq!(combined_value, String::from("Hello world!!"));
    /// }
    /// ```
    fn append_and_return(self: Self, string: &str) -> Self {
        let mut return_value = self;
        return_value.push_str(string);
        return_value
    }
}
