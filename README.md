# Is Number

A Rust library to check if a string represents a valid number.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
is_number = "0.1.0"
```

## Examples

```rust
use is_number::is_number;

fn main() {
assert!(is_number("123"));
assert!(is_number("-123.456"));
assert!(is_number("1.23e-5"));
}
```
