# appendreturn

This crate is for anyone who is annoyed with the fact that Rust's `.push_str()` method for `String` does not return the resulting value. 

This crate is here for you. Hope this helps. ❤️

## License

This software is dedicated into the public domain. See [UNLICENSE](./UNLICENSE) for details. 

## Example Usage

```rust
use appendreturn::AppendReturn;
let original_value = String::from("Hello");
let additional_value = " world!!";
let combined_value = original_value.append_and_return(additional_value);
assert_eq!(combined_value, String::from("Hello world!!"));
```