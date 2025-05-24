use serde_json::{json, Value};
use validate_ro::rules::Rule;
use validate_ro::traits::Validator;
use validate_ro::{FormValidator, Rules};
use validate_ro::error::ValidationError;

#[test]
fn test_rules_validation() {
    let rules = Rules::new().add(Rule::required()).add(Rule::min_length(3));

    // Test valid input
    assert!(rules.validate(&json!("valid")).is_ok());

    // Test empty string
    assert!(rules.validate(&json!("")).is_err());

    // Test null value
    assert!(rules.validate(&Value::Null).is_err());

    // Test too short string
    assert!(rules.validate(&json!("ab")).is_err());
}

#[test]
fn test_form_validator_success() {
    let form_validator = FormValidator::new()
        .add("username", Rules::new().add(Rule::required()).add(Rule::min_length(3)))
        .add("age", Rules::new().add(Rule::integer()).add(Rule::max_value(120.0)));

    let form_data = json!({
        "username": "testuser",
        "age": 25
    });

    let result = form_validator.validate(&form_data);
    assert!(result.is_ok());

    let valid_data = result.unwrap();
    assert_eq!(valid_data["username"], "testuser");
    assert_eq!(valid_data["age"], 25);
}

#[test]
fn test_form_validator_with_errors() {
    let form_validator = FormValidator::new()
        .add("email", Rules::new().add(Rule::required()).add(Rule::email(None)))
        .add("password", Rules::new().add(Rule::required()).add(Rule::min_length(8)));

    let form_data = json!({
        "email": "invalid-email",
        "password": "short"
    });

    let result = form_validator.validate(&form_data);
    assert!(result.is_err());

    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 2);


    assert!(matches!(errors.get("email").unwrap().get(0).unwrap(), ValidationError::EmailError(_)));
    assert!(matches!(errors.get("password").unwrap().get(0).unwrap(), ValidationError::MinLengthError{..}));
}

#[test]
fn test_form_validator_missing_required_field() {
    let form_validator = FormValidator::new()
        .add("username", Rules::new().add(Rule::required()))
        .add("email", Rules::new().add(Rule::required()));

    let form_data = json!({
        "username": "testuser"
        // email is missing
    });

    let result = form_validator.validate(&form_data);
    assert!(result.is_err());

    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);
    assert!(matches!(errors.get("email").unwrap().get(0).unwrap(), ValidationError::Required));
}

#[test]
fn test_form_validator_break_on_first_error() {
    let form_validator = FormValidator::break_on_first_error()
        .add("email", Rules::new().add(Rule::email(None)))
        .add("password", Rules::new().add(Rule::required()).add(Rule::min_length(8)));

    let form_data = json!({
        "email": "invalid-email",
        "password": "short"
    });

    let result = form_validator.validate(&form_data);
    assert!(result.is_err());

    // Should only return the first error
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);
    assert!(matches!(errors.get("email").unwrap().get(0).unwrap(), ValidationError::EmailError(_)));
}

#[test]
fn test_nested_rules_validation() {
    let form_validator = FormValidator::new()
        .add("user", Rules::new().add(Rule::required()).add(Rule::min_length(3)))
        .add("settings.notifications", Rules::new().add(Rule::required()));

    // Test with nested valid data
    let valid_data = json!({
        "user": "testuser",
        "settings": {
            "notifications": true
        }
    });
    assert!(form_validator.validate(&valid_data).is_ok());
    // Test with invalid nested data
    let invalid_data = json!({
        "user": "tu",  // too short
        "settings": {}  // missing notifications
    });
    let result = form_validator.validate(&invalid_data);
    assert!(result.is_err());

    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 2);
}

#[test]
fn test_custom_validator_in_form() {
    let custom_validator = |value: &Value| {
        if let Some(s) = value.as_str() {
            if s.chars().any(|c| c.is_ascii_uppercase()) {
                Ok(())
            } else {
                Err(ValidationError::Custom(
                    "Must contain uppercase".to_string(),
                ))
            }
        } else {
            Ok(())
        }
    };

    let form_validator = FormValidator::new().add(
        "password",
        Rules::new()
            .add(Rule::required())
            .add(Rule::min_length(8))
            .add(custom_validator),
    );

    // Test valid password
    let valid_data = json!({
        "password": "SecurePass123"
    });
    assert!(form_validator.validate(&valid_data).is_ok());

    // Test invalid password (no uppercase)
    let invalid_data = json!({
        "password": "weakpass123"
    });
    let result = form_validator.validate(&invalid_data);
    assert!(result.is_err());
    match result {
        Ok(_) => {}
        Err(errors) => {
            assert!(matches!(errors.get("password").unwrap().get(0).unwrap(), ValidationError::Custom(_)));
        }
    }
}
#[test]
fn test_default_validator_in_form() {

    let form_validator = FormValidator::new().add(
        "name",
        Rules::new()
            .add(Rule::required())
            .add(Rule::string())
    ).add("active",Rules::new().add(Rule::boolean()).default(Value::Bool(false)));

    // Test valid password
    let valid_data = json!({
        "name": "Ali"
    });
    let data=form_validator.validate(&valid_data);

    match data {
        Ok(d) => {
            assert_eq!(d.get("active").unwrap(),&Value::Bool(false));
        }
        Err(errors) => {
            panic!("error: {:?}",errors);
        }
    }
}
