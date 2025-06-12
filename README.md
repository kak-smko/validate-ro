# Validation Library

[![Crates.io](https://img.shields.io/crates/v/validate-ro)](https://crates.io/crates/validate-ro)
[![Documentation](https://docs.rs/validate-ro/badge.svg)](https://docs.rs/validate-ro)
[![License](https://img.shields.io/crates/l/validate-ro)](LICENSE-MIT)


A flexible, extensible validation framework for Rust with support for both synchronous and asynchronous validation, including MongoDB integration for unique checks.

## Features

- 🛠️ **40+ built-in validators** for common validation scenarios
- ⚡ **Async support** for database-backed validation
- 🏗️ **Composable rules** with fluent builder pattern
- 📝 **Nested field validation** using dot notation
- 🧩 **Custom validators** for specialized requirements
- 🗃️ **MongoDB integration** for unique field validation
- 🏷️ **Default values** for missing fields
- 🚦 **Flexible error handling** with detailed error messages

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
validate-ro = "0.1"
```


## Quick Start

```rust
use validate_ro::{FormValidator,Rules,rules, rules::Rule};
use serde_json::json;

fn main() {
    // Build a validator
    let validator = FormValidator::new()
        .add("username", Rules::new().add(Rule::required()).add(Rule::min_length(5)))
        .add("email", rules![Rule::required(),Rule::email(None)])
        .add("age", Rules::new().add(Rule::integer()).add(Rule::min_value(18.0)).default(json!(21)));

    // Validate some data
    let data = json!({
        "username": "user123",
        "email": "test@example.com"
    });

    match validator.validate(&data) {
        Ok(valid_data) => {
            println!("Valid data: {:?}", valid_data);  // age will be 21 (default)
        },
        Err(errors) => {
            eprintln!("Validation errors: {:?}", errors);
        }
    }
}
```

## Core Concepts

### 1. Basic Validation

```rust
use validate_ro::Rules;
use validate_ro::rules::Rule;

// Single field validation
let is_adult = Rules::new().add(Rule::integer()).add(Rule::min_value(18.0));
assert!(is_adult.validate(&json!(21)).is_ok());

// or use macro
let is_adult = rules![Rule::integer(),Rule::min_value(18.0)];
assert!(is_adult.validate(&json!(21)).is_ok());
```

### 2. Combining Rules

```rust
let password_validator = Rules::new().add(Rule::required())
    .add(Rule::min_length(8))
    .add(Rule::regex(r"[A-Z]", Some("Must contain uppercase".into())).unwrap());
```

### 3. Form Validation

```rust
let form_validator = FormValidator::new()
    .add("user.name", Rule::required())
    .add("user.email", Rule::email(None))
    .add("user.age", Rule::integer());
```

### 4. Async Validation (MongoDB)

```rust
#[tokio::main]
async fn main() {
    let client = mongodb::Client::with_uri_str("mongodb://localhost:27017").await.unwrap();
    let db = Arc::new(client.database("test"));
    
    let validator = FormValidator::new()
        .add("email", Rule::unique("users", "email", None));

    let result = validator.validate_async(&db, &json!({"email": "user@example.com"})).await;
}
```

## Available Validators

### Basic Validators
- `required()` - Field must be present and not null
- `string()` - Must be a string
- `integer()` - Must be an integer
- `float()` - Must be a float
- `boolean()` - Must be a boolean
- `array()` - Must be an array
- `object()` - Must be an object

### String Validators
- `length(n)` - Exact length
- `min_length(n)` - Minimum length
- `max_length(n)` - Maximum length
- `email()` - Valid email format
- `url()` - Valid URL format
- `ip()` - Valid IP address
- `regex()` - Matches regex pattern
- `accepted()` - Common "accepted" terms (true, 1, "yes", "on")

### Numeric Validators
- `min_value(n)` - Minimum value
- `max_value(n)` - Maximum value
- `equal(n)` - Exact value match
- `numeric()` - Can be parsed as number

### Collection Validators
- `in_values()` - Value must be in allowed set
- `not_in_values()` - Value must not be in excluded set

### Database Validators
- `unique()` - Field value must be unique in MongoDB collection

### File Validators
- `extensions()` - File extension must be in allowed set

## Advanced Usage

### Custom Validators

```rust
let validator = Rule::custom(|value| {
    if let Some(s) = value.as_str() {
        if s.starts_with("prefix_") {
            Ok(())
        } else {
            Err(ValidationError::Custom("Must start with 'prefix_'".into()))
        }
    } else {
        Err(ValidationError::TypeError {
            expected: "string".into(),
            got: value.to_string()
        })
    }
});
```

### Error Handling

```rust
match validator.validate(&data) {
    Ok(valid_data) => { /* handle success */ },
    Err(errors) => {
        for (field, field_errors) in errors {
            println!("Field '{}' errors:", field);
            for error in field_errors {
                println!("- {}", error);
            }
        }
    }
}
```


## Performance

The library is designed for efficiency:
- Minimal allocations
- Lazy validation (stops on first error when configured)
- Thread-safe by design


## Contributing

Contributions are welcome! Please open an issue or submit a PR for:
- New features
- Performance improvements
- Bug fixes


## License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE) at your option.