# soundex-rs
A library that calculates soundex value.

# usage
```rust
use soundex_rs::Soundex;

let value = "hello world".soundex();
assert_eq!(value, "H464".to_string());
```

# features
- default: The result retains the first four characters of the soundex value
- full: The result retains the complete value of soundex

# reference
- https://support.esri.com/en/technical-article/000003773
