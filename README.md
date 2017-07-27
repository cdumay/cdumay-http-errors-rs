# cdumay-http-errors

A rust lib for error JSON serialization

## Installation

This crate works  works with Cargo:
    
```toml
[dependencies]
cdumay-http-errors = "0.1"
```

## Usage

Link the library in _main.rs_:

```rust
#[macro_use]
extern crate cdumay_http_errors;
```

Example of use:

```rust
use cdumay_http_errors::HttpCode;

println!("{}", http_error!(HttpCode::OK, "A useful result!"));
```

## License

cdumay-http-errors is licensed under MIT license(see [LICENSE](LICENSE) or http://opensource.org/licenses/MIT)