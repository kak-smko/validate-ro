//! # Validation Library
//!
//! A flexible, extensible validation framework with support for:
//! - Synchronous and asynchronous validation
//! - Complex nested field validation
//! - Custom validation rules
//! - MongoDB integration for unique checks
//! - Default values and error accumulation
//!
//! ## Core Concepts
//!
//! 1. **Validators**: Implement the `Validator` trait to create validation rules
//! 2. **Rules**: Combine multiple validators with optional default values
//! 3. **FormValidator**: Validate complete forms/objects with field-level rules
//!
//! ## Example: Basic Usage
//!
//! ```rust
//! use validate_ro::{Rules, FormValidator};
//! use serde_json::json;
//! use validate_ro::rules::Rule;
//!
//! // Create validation rules
//! let email_rule = Rules::new()
//!     .add(Rule::required())
//!     .add(Rule::email(None));
//!
//! let age_rule = Rules::new()
//!     .add(Rule::integer())
//!     .add(Rule::min_value(18.0))
//!     .default(json!(21)); // Default value if null
//!
//! // Build form validator
//! let validator = FormValidator::new()
//!     .add("email", email_rule)
//!     .add("age", age_rule);
//!
//! // Validate data
//! let data = json!({"email": "test@example.com"});
//! match validator.validate(&data) {
//!     Ok(valid_data) => {
//!         // age will be 21 (default value)
//!         println!("Valid data: {:?}", valid_data);
//!     },
//!     Err(errors) => {
//!         eprintln!("Validation errors: {:?}", errors);
//!     }
//! }
//! ```

use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;
use mongodb::Database;
use serde_json::Value;
use crate::error::ValidationError;
use crate::traits::{ValidationResult, Validator};

pub mod rules;
pub mod traits;
pub mod error;

/// Container for multiple validators with optional default value
///
/// # Examples
///
/// ```
/// use validate_ro::{Rules};
/// use serde_json::json;
/// use validate_ro::rules::Rule;
///
/// let rule = Rules::new()
///     .add(Rule::required())
///     .add(Rule::min_length(8))
///     .default(json!("default"));
/// ```
pub struct Rules {
    validators: Vec<Box<dyn Validator+ Send + Sync>>,
    default_value: Option<Value>,
}

impl Rules {
    /// Creates a new empty Rules container
    pub fn new() -> Self {
        Self {
            validators: Vec::new(),
            default_value: None,
        }
    }

    /// Adds a validator to the rules chain
    pub fn add<V: Validator + 'static>(mut self, validator: V) -> Self {
        self.validators.push(Box::new(validator));
        self
    }

    /// Sets a default value that will be used when input is null
    pub fn default(mut self, default: Value) -> Self {
        self.default_value = Some(default);
        self
    }
}

impl Validator for Rules {
    fn validate(&self, value: &Value) -> ValidationResult {
        let value = if value.is_null() && self.default_value.is_some() {
            self.default_value.as_ref().unwrap()
        } else {
            value
        };
        for validator in &self.validators {
            validator.validate(value)?;
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}


/// Validates complete forms/objects with field-level rules
///
/// Supports:
/// - Nested field paths (e.g., "user.address.street")
/// - Early termination on first error
/// - Async validation with MongoDB
///
/// # Example
///
/// ```
/// use validate_ro::{FormValidator};
/// use validate_ro::rules::Rule;
///
/// let validator = FormValidator::new()
///     .add("username", Rule::required())
///     .add("profile.age", Rule::integer());
/// ```
pub struct FormValidator {
    break_on_error:bool,
    field_validators: HashMap<String, Box<dyn Validator+ Send + Sync>>,
}

impl FormValidator {
    /// Creates a new validator that collects all errors
    pub fn new() -> Self {
        Self {
            break_on_error:false,
            field_validators: HashMap::new(),
        }
    }
    /// Creates a validator that stops after first error
    pub fn break_on_first_error()->Self{
        Self {
            break_on_error:true,
            field_validators: HashMap::new(),
        }
    }

    /// Adds validation rules for a field
    ///
    /// # Arguments
    ///
    /// * `field_name` - Field path (supports dot notation for nested fields)
    /// * `validator` - Validation rules
    pub fn add(
        mut self,
        field_name: &str,
        validator: impl Validator + 'static,
    ) -> Self {
        self.field_validators
            .insert(field_name.to_string(), Box::new(validator));
        self
    }

    /// Validates form data synchronously
    ///
    /// Returns either:
    /// - Ok(HashMap) with validated values (including defaults)
    /// - Err(HashMap) with field names and error lists
    pub fn validate(
        &self,
        form_data: &Value,
    ) -> Result<HashMap<String, Value>, HashMap<String,Vec<ValidationError>>> {
        let mut errors = HashMap::new();
        let mut valid_data = HashMap::new();

        for (field_name, validator) in &self.field_validators {
            let value = if field_name.contains('.') {
                let mut current = form_data;
                for part in field_name.split('.') {
                    current = current.get(part).unwrap_or(&Value::Null);
                }
                current
            } else {
                form_data.get(field_name).unwrap_or(&Value::Null)
            };
            let processed_value = if let Some(rules) = validator.as_any().downcast_ref::<Rules>() {
                if value.is_null() && rules.default_value.is_some() {
                    rules.default_value.as_ref().unwrap()
                } else {
                    value
                }
            } else {
                value
            };
            if let Err(err) = validator.validate(processed_value) {
                match errors.get_mut(field_name){
                    None => {
                        errors.insert(field_name.clone(),vec![err]);
                    }
                    Some(a) => {
                        a.push(err);
                    }
                }

                if self.break_on_error {
                    break;
                }
            } else {
                valid_data.insert(field_name.clone(), processed_value.clone());
            }
        }

        if errors.is_empty() {
            Ok(valid_data)
        } else {
            Err(errors)
        }
    }

    /// Validates form data asynchronously with MongoDB access
    ///
    /// Used for validators that require database checks (like uniqueness)
    pub async fn validate_async(
        &self,
        db:&Arc<Database>,
        form_data: &Value,
    ) -> Result<HashMap<String, Value>, HashMap<String,Vec<ValidationError>>> {
        let mut errors = HashMap::new();
        let mut valid_data = HashMap::new();

        for (field_name, validator) in &self.field_validators {
            let value = if field_name.contains('.') {
                let mut current = form_data;
                for part in field_name.split('.') {
                    current = current.get(part).unwrap_or(&Value::Null);
                }
                current
            } else {
                form_data.get(field_name).unwrap_or(&Value::Null)
            };
            let processed_value = if let Some(rules) = validator.as_any().downcast_ref::<Rules>() {
                if value.is_null() && rules.default_value.is_some() {
                    rules.default_value.as_ref().unwrap()
                } else {
                    value
                }
            } else {
                value
            };
            if let Err(err) = validator.validate_async(db,processed_value).await {
                match errors.get_mut(field_name){
                    None => {
                        errors.insert(field_name.clone(),vec![err]);
                    }
                    Some(a) => {
                        a.push(err);
                    }
                }
                if self.break_on_error {
                    break;
                }
            } else {
                valid_data.insert(field_name.clone(), processed_value.clone());
            }
        }

        if errors.is_empty() {
            Ok(valid_data)
        } else {
            Err(errors)
        }
    }
}