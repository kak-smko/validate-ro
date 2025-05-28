use validate_ro::rules::*;
use serde_json::{json, Value};
use validate_ro::error::ValidationError;
use validate_ro::traits::Validator;

#[test]
fn test_required() {
    let validator = Rule::required();
    assert!(validator.validate(&Value::Null).is_err());
    assert!(validator.validate(&json!("test")).is_ok());
}

#[test]
fn test_type_validators() {
    // String
    let string_validator = Rule::string();
    assert!(string_validator.validate(&json!("test")).is_ok());
    assert!(string_validator.validate(&json!(123)).is_err());
    assert!(string_validator.validate(&Value::Null).is_ok());

    // Boolean
    let bool_validator = Rule::boolean();
    assert!(bool_validator.validate(&json!(true)).is_ok());
    assert!(bool_validator.validate(&json!("true")).is_err());
    assert!(bool_validator.validate(&Value::Null).is_ok());

    // Float
    let float_validator = Rule::float();
    assert!(float_validator.validate(&json!(1.23)).is_ok());
    assert!(float_validator.validate(&json!(123)).is_err()); // i64 not f64
    assert!(float_validator.validate(&Value::Null).is_ok());

    // Integer
    let int_validator = Rule::integer();
    assert!(int_validator.validate(&json!(123)).is_ok());
    assert!(int_validator.validate(&json!(1.23)).is_err());
    assert!(int_validator.validate(&Value::Null).is_ok());
}

#[test]
fn test_length_validators() {
    // Length
    let length_validator = Rule::length(3);
    assert!(length_validator.validate(&json!("abc")).is_ok());
    assert!(length_validator.validate(&json!("abcd")).is_err());
    assert!(length_validator.validate(&json!([1, 2, 3])).is_ok());
    assert!(length_validator.validate(&Value::Null).is_ok());

    // Min length
    let min_len_validator = Rule::min_length(3);
    assert!(min_len_validator.validate(&json!("abc")).is_ok());
    assert!(min_len_validator.validate(&json!("ab")).is_err());
    assert!(min_len_validator.validate(&Value::Null).is_ok());

    // Max length
    let max_len_validator = Rule::max_length(3);
    assert!(max_len_validator.validate(&json!("abc")).is_ok());
    assert!(max_len_validator.validate(&json!("abcd")).is_err());
    assert!(max_len_validator.validate(&Value::Null).is_ok());
}

#[test]
fn test_numeric_validators() {
    // Equal
    let equal_validator = Rule::equal(json!(42));
    assert!(equal_validator.validate(&json!(42)).is_ok());
    assert!(equal_validator.validate(&json!(43)).is_err());
    assert!(equal_validator.validate(&Value::Null).is_ok());

    // Min value
    let min_val_validator = Rule::min_value(10.0);
    assert!(min_val_validator.validate(&json!(10.0)).is_ok());
    assert!(min_val_validator.validate(&json!(9.9)).is_err());
    assert!(min_val_validator.validate(&Value::Null).is_ok());

    // Max value
    let max_val_validator = Rule::max_value(10.0);
    assert!(max_val_validator.validate(&json!(10.0)).is_ok());
    assert!(max_val_validator.validate(&json!(10.1)).is_err());
    assert!(max_val_validator.validate(&Value::Null).is_ok());

    // Numeric
    let numeric_validator = Rule::numeric();
    assert!(numeric_validator.validate(&json!("123")).is_ok());
    assert!(numeric_validator.validate(&json!("abc")).is_err());
    assert!(numeric_validator.validate(&Value::Null).is_ok());
}

#[test]
fn test_accepted() {
    let accepted_validator = Rule::accepted();
    assert!(accepted_validator.validate(&json!("yes")).is_ok());
    assert!(accepted_validator.validate(&json!("YES")).is_ok());
    assert!(accepted_validator.validate(&json!(true)).is_ok());
    assert!(accepted_validator.validate(&json!(1)).is_ok());
    assert!(accepted_validator.validate(&json!("no")).is_err());
    assert!(accepted_validator.validate(&Value::Null).is_ok());
}

#[test]
fn test_email() {
    let email_validator = Rule::email(None);
    assert!(email_validator.validate(&json!("test@example.com")).is_ok());
    assert!(email_validator.validate(&json!("invalid")).is_err());
    assert!(email_validator.validate(&json!("a@b.c")).is_err()); // name too short
    assert!(email_validator.validate(&Value::Null).is_ok());

    let restricted_email = Rule::email(Some(vec!["example.com".to_string()]));
    assert!(
        restricted_email
            .validate(&json!("test@example.com"))
            .is_ok()
    );
    assert!(restricted_email.validate(&json!("test@other.com")).is_err());
}

#[test]
fn test_in_values() {
    let in_validator = Rule::in_values(vec![json!(1), json!("two"), json!(true)]);
    assert!(in_validator.validate(&json!(1)).is_ok());
    assert!(in_validator.validate(&json!("two")).is_ok());
    assert!(in_validator.validate(&json!(true)).is_ok());
    assert!(in_validator.validate(&json!(2)).is_err());
    assert!(in_validator.validate(&Value::Null).is_ok());
}

#[test]
fn test_not_in_values() {
    let not_in_validator = Rule::not_in_values(vec![json!(1), json!("two"), json!(true)]);
    assert!(not_in_validator.validate(&json!(2)).is_ok());
    assert!(not_in_validator.validate(&json!("three")).is_ok());
    assert!(not_in_validator.validate(&json!(false)).is_ok());
    assert!(not_in_validator.validate(&json!(1)).is_err());
    assert!(not_in_validator.validate(&Value::Null).is_ok());
}

#[test]
fn test_regex() {
    let regex_validator = Rule::regex(r"^\d+$", None).unwrap();
    assert!(regex_validator.validate(&json!("123")).is_ok());
    assert!(regex_validator.validate(&json!("abc")).is_err());
    assert!(regex_validator.validate(&Value::Null).is_ok());

    let with_message = Rule::regex(r"^\d+$", Some("Must be digits".to_string())).unwrap();
    if let Err(ValidationError::RegexError(msg)) = with_message.validate(&json!("abc")) {
        assert_eq!(msg, "Must be digits");
    } else {
        panic!("Expected RegexError with custom message");
    }
}

#[test]
fn test_url() {
    let url_validator = Rule::url();
    assert!(
        url_validator
            .validate(&json!("https://example.com"))
            .is_ok()
    );
    assert!(url_validator.validate(&json!("invalid")).is_err());
    assert!(url_validator.validate(&Value::Null).is_ok());
}

#[test]
fn test_ip() {
    let ip_validator = Rule::ip();
    assert!(ip_validator.validate(&json!("192.168.1.1")).is_ok());
    assert!(ip_validator.validate(&json!("256.168.1.1")).is_err());
    assert!(ip_validator.validate(&json!("not.an.ip")).is_err());
    assert!(ip_validator.validate(&Value::Null).is_ok());
}

#[test]
fn test_extensions() {
    let ext_validator = Rule::extensions(vec!["jpg".to_string(), "png".to_string()]);
    assert!(ext_validator.validate(&json!("image.jpg")).is_ok());
    assert!(ext_validator.validate(&json!("file.png")).is_ok());
    assert!(ext_validator.validate(&json!("document.pdf")).is_err());
    assert!(ext_validator.validate(&Value::Null).is_ok());
}

#[test]
fn test_custom_validator() {
    let custom_validator = Rule::custom(|value: &Value| {
        if let Value::String(s) = value {
            if s.len() > 5 {
                Ok(())
            } else {
                Err(ValidationError::Custom("Too short".to_string()))
            }
        } else {
            Ok(())
        }
    });

    assert!(custom_validator.validate(&json!("long enough")).is_ok());
    assert!(custom_validator.validate(&json!("short")).is_err());
    assert!(custom_validator.validate(&Value::Null).is_ok());
}
