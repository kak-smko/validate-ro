//! # Built-in Validation Rules
//!
//! Provides common validation rules ready to use:
//!
//! ## Basic Type Validation
//! - `required()` - Value must not be null
//! - `string()` - Value must be a string
//! - `integer()` - Value must be an integer
//! - `float()` - Value must be a float
//! - `boolean()` - Value must be a boolean
//! - `array()` - Value must be an array
//! - `object()` - Value must be an object
//!
//! ## String Validation
//! - `length(n)` - Exact length
//! - `min_length(n)` - Minimum length
//! - `max_length(n)` - Maximum length
//! - `email()` - Valid email format
//! - `url()` - Valid URL format
//! - `ip()` - Valid IP address
//! - `regex()` - Matches regex pattern
//!
//! ## Numeric Validation
//! - `min_value(n)` - Minimum numeric value
//! - `max_value(n)` - Maximum numeric value
//! - `equal(n)` - Exact value match
//!
//! ## Collection Validation
//! - `in_values()` - Value must be in allowed set
//! - `not_in_values()` - Value must not be in excluded set
//!
//! ## Database Validation
//! - `unique()` - Field value must be unique in MongoDB collection
//!
//! ## Custom Validation
//! - `custom()` - Implement custom validation logic

use std::any::Any;
use std::collections::HashSet;
use std::sync::Arc;
use async_trait::async_trait;
use mongodb::bson::{doc, Bson, Document};
use mongodb::{Collection, Database};
use mongodb::bson::oid::ObjectId;
use regex::Regex;
use serde_json::Value;
use crate::error::ValidationError;
use crate::traits::{ValidationResult, Validator};

/// Factory for creating validation rules
pub struct Rule;
impl Rule {
    /// Validates that value is not null
    ///
    /// # Example
    ///
    /// ```
    /// use validate_ro::rules::Rule;
    ///
    /// let validator = Rule::required();
    /// ```
    pub fn required() -> impl Validator {
        move |value: &Value| {
            if value.is_null() {
                Err(ValidationError::Required)
            } else {
                Ok(())
            }
        }
    }

    /// Validates that value is a string (or null)
    pub fn string() -> impl Validator {
        move |value: &Value| {
            if value.is_null() {
                return Ok(())
            }
            if !value.is_string() {
                Err(ValidationError::TypeError {
                    expected: "string".to_string(),
                    got: value.to_string()
                })
            } else {
                Ok(())
            }
        }
    }

    /// Validates that value is an array (or null)
    ///
    /// # Example
    ///
    /// ```
    /// use serde_json::json;
    /// use validate_ro::rules::Rule;
    ///
    /// let validator = Rule::array();
    /// assert!(validator.validate(&json!([1, 2, 3])).is_ok());
    /// ```
    pub fn array() -> impl Validator {
        move |value: &Value| {
            if value.is_null() {
                return Ok(())
            }
            if !value.is_array() {
                Err(ValidationError::TypeError {
                    expected: "array".to_string(),
                    got: value.to_string()
                })
            } else {
                Ok(())
            }
        }
    }

    /// Validates that value is an object (or null)
    ///
    /// # Example
    ///
    /// ```
    /// use serde_json::json;
    /// use validate_ro::rules::Rule;
    ///
    /// let validator = Rule::object();
    /// assert!(validator.validate(&json!({"key": "value"})).is_ok());
    /// ```
    pub fn object() -> impl Validator {
        move |value: &Value| {
            if value.is_null() {
                return Ok(())
            }
            if !value.is_object() {
                Err(ValidationError::TypeError {
                    expected: "object".to_string(),
                    got: value.to_string()
                })
            } else {
                Ok(())
            }
        }
    }

    /// Validates that value is a boolean (or null)
    ///
    /// # Example
    ///
    /// ```
    /// use serde_json::json;
    /// use validate_ro::rules::Rule;
    ///
    /// let validator = Rule::boolean();
    /// assert!(validator.validate(&json!(true)).is_ok());
    /// ```
    pub fn boolean() -> impl Validator {
        move |value: &Value| {
            if value.is_null() {
                return Ok(())
            }
            if !value.is_boolean() {
                Err(ValidationError::TypeError {
                    expected: "bool".to_string(),
                    got: value.to_string()
                })
            } else {
                Ok(())
            }
        }
    }

    /// Validates that value is a float number (or null)
    ///
    /// # Example
    ///
    /// ```
    /// use serde_json::json;
    /// use validate_ro::rules::Rule;
    ///
    /// let validator = Rule::float();
    /// assert!(validator.validate(&json!(3.14)).is_ok());
    /// ```
    pub fn float() -> impl Validator {
        move |value: &Value| {
            if value.is_null() {
                return Ok(())
            }
            if !value.is_f64() {
                Err(ValidationError::TypeError {
                    expected: "float".to_string(),
                    got: value.to_string()
                })
            } else {
                Ok(())
            }
        }
    }

    /// Validates that value is an integer (or null)
    ///
    /// # Example
    ///
    /// ```
    /// use serde_json::json;
    /// use validate_ro::rules::Rule;
    ///
    /// let validator = Rule::integer();
    /// assert!(validator.validate(&json!(42)).is_ok());
    /// ```
    pub fn integer() -> impl Validator {
        move |value: &Value| {
            if value.is_null() {
                return Ok(())
            }
            if !value.is_i64() {
                Err(ValidationError::TypeError {
                    expected: "int".to_string(),
                    got: value.to_string()
                })
            } else {
                Ok(())
            }
        }
    }


    /// Validates exact length for strings/arrays/objects
    ///
    /// # Arguments
    ///
    /// * `len` - Exact required length
    ///
    /// # Example
    ///
    /// ```
    /// use serde_json::json;
    /// use validate_ro::rules::Rule;
    ///
    /// let validator = Rule::length(3);
    /// assert!(validator.validate(&json!("abc")).is_ok());
    /// assert!(validator.validate(&json!([1, 2, 3])).is_ok());
    /// ```
    pub fn length(len: usize) -> impl Validator {
        LengthValidator { length: len }
    }


    /// Validates minimum length for strings/arrays/objects
    ///
    /// # Arguments
    ///
    /// * `min` - Minimum allowed length
    ///
    /// # Example
    ///
    /// ```
    /// use serde_json::json;
    /// use validate_ro::rules::Rule;
    ///
    /// let validator = Rule::min_length(5);
    /// assert!(validator.validate(&json!("long enough")).is_ok());
    /// ```
    pub fn min_length(min: usize) -> impl Validator {
        MinLengthValidator { min }
    }


    /// Validates maximum length for strings/arrays/objects
    ///
    /// # Arguments
    ///
    /// * `max` - Maximum allowed length
    ///
    /// # Example
    ///
    /// ```
    /// use serde_json::json;
    /// use validate_ro::rules::Rule;
    ///
    /// let validator = Rule::max_length(10);
    /// assert!(validator.validate(&json!("short")).is_ok());
    /// ```
    pub fn max_length(max: usize) -> impl Validator {
        MaxLengthValidator { max }
    }


    /// Validates exact value match
    ///
    /// # Arguments
    ///
    /// * `value` - Expected value to match against
    ///
    /// # Example
    ///
    /// ```
    /// use serde_json::json;
    /// use validate_ro::rules::Rule;
    ///
    /// let validator = Rule::equal(json!("expected"));
    /// assert!(validator.validate(&json!("expected")).is_ok());
    /// ```
    pub fn equal(value: Value) -> impl Validator {
        EqualValidator { value }
    }


    /// Validates minimum numeric value
    ///
    /// # Arguments
    ///
    /// * `min` - Minimum allowed value
    ///
    /// # Example
    ///
    /// ```
    /// use serde_json::json;
    /// use validate_ro::rules::Rule;
    ///
    /// let validator = Rule::min_value(18.0);
    /// assert!(validator.validate(&json!(21)).is_ok());
    /// ```
    pub fn min_value(min: f64) -> impl Validator {
        MinValueValidator { min }
    }


    /// Validates maximum numeric value
    ///
    /// # Arguments
    ///
    /// * `max` - Maximum allowed value
    ///
    /// # Example
    ///
    /// ```
    /// use serde_json::json;
    /// use validate_ro::rules::Rule;
    ///
    /// let validator = Rule::max_value(100.0);
    /// assert!(validator.validate(&json!(75)).is_ok());
    /// ```
    pub fn max_value(max: f64) -> impl Validator {
        MaxValueValidator { max }
    }


    /// Validates that string can be parsed as number (or null)
    ///
    /// # Example
    ///
    /// ```
    /// use serde_json::json;
    /// use validate_ro::rules::Rule;
    ///
    /// let validator = Rule::numeric();
    /// assert!(validator.validate(&json!("123.45")).is_ok());
    /// ```
    pub fn numeric() -> impl Validator {
        move |value: &Value| {
            if value.is_null() {
                return Ok(())
            }
            if let Value::String(s) = value {
                if s.parse::<f64>().is_ok() {
                    return Ok(());
                }
            }
            Err(ValidationError::NumericError(value.to_string()))
        }
    }

    /// Validates common "accepted" terms (true, 1, "yes", "on")
    ///
    /// # Example
    ///
    /// ```
    /// use serde_json::json;
    /// use validate_ro::rules::Rule;
    ///
    /// let validator = Rule::accepted();
    /// assert!(validator.validate(&json!("yes")).is_ok());
    /// assert!(validator.validate(&json!(true)).is_ok());
    /// ```
    pub fn accepted() -> impl Validator {
        move |value: &Value| {
            if value.is_null() {
                return Ok(())
            }
            let s = match value {
                Value::String(s) => s.to_lowercase(),
                Value::Bool(b) => b.to_string(),
                Value::Number(n) => n.to_string(),
                _ => return Err(ValidationError::TypeError {
                    expected: "string, bool, or number".to_string(),
                    got: value.to_string(),
                }),
            };

            if matches!(s.as_str(), "yes" | "on" | "1" | "true") {
                Ok(())
            } else {
                Err(ValidationError::AcceptedError(value.to_string()))
            }
        }
    }

    /// Validates that string matches email format
    ///
    /// # Arguments
    ///
    /// * `allowed_domains` - Optional list of allowed email domains
    ///
    /// # Example
    ///
    /// ```
    /// use validate_ro::rules::Rule;
    ///
    /// // Only allow @company.com emails
    /// let validator = Rule::email(Some(vec!["company.com".to_string()]));
    /// ```

    pub fn email(allowed_domains: Option<Vec<String>>) -> impl Validator {
        EmailValidator {
            allowed_domains: allowed_domains.map(|v| v.into_iter().collect()),
        }
    }

    /// Validates that value is in allowed set
    ///
    /// # Arguments
    ///
    /// * `values` - Allowed values
    ///
    /// # Example
    ///
    /// ```
    /// use serde_json::json;
    /// use validate_ro::rules::Rule;
    ///
    /// let validator = Rule::in_values(vec![json!("red"), json!("blue")]);
    /// assert!(validator.validate(&json!("red")).is_ok());
    /// ```
    pub fn in_values(values: Vec<Value>) -> impl Validator {
        InValidator {
            values
        }
    }


    /// Validates that value is not in excluded set
    ///
    /// # Arguments
    ///
    /// * `values` - Excluded values
    ///
    /// # Example
    ///
    /// ```
    /// use serde_json::json;
    /// use validate_ro::rules::Rule;
    ///
    /// let validator = Rule::not_in_values(vec![json!("admin")]);
    /// assert!(validator.validate(&json!("user")).is_ok());
    /// ```
    pub fn not_in_values(values: Vec<Value>) -> impl Validator {
        NotInValidator {
            values
        }
    }

    /// Validates string against regex pattern
    ///
    /// # Arguments
    ///
    /// * `pattern` - Regex pattern
    /// * `message` - Optional custom error message
    ///
    /// # Example
    ///
    /// ```
    /// use serde_json::json;
    /// use validate_ro::rules::Rule;
    ///
    /// let validator = Rule::regex(r"^\d{3}-\d{3}$", None).unwrap();
    /// assert!(validator.validate(&json!("123-456")).is_ok());
    /// ```
    pub fn regex(pattern: &str, message: Option<String>) -> Result<impl Validator, regex::Error> {
        Ok(RegexValidator {
            pattern: Regex::new(pattern)?,
            message,
        })
    }

    /// Validates that value is a valid URL (or null)
    ///
    /// # Example
    ///
    /// ```
    /// use serde_json::json;
    /// use validate_ro::rules::Rule;
    ///
    /// let validator = Rule::url();
    /// assert!(validator.validate(&json!("https://example.com")).is_ok());
    /// ```
    pub fn url() -> impl Validator {
        move |value: &Value| {
            if value.is_null() {
                return Ok(())
            }
            let s = match value {
                Value::String(s) => s,
                _ => return Err(ValidationError::TypeError {
                    expected: "string".to_string(),
                    got: value.to_string(),
                }),
            };

            let pattern = r#"(?i)\b((?:https?://|www\d{0,3}[.]|[a-z0-9.\-]+[.][a-z]{2,4}/)(?:[^\s()<>]+|\(([^\s()<>]+|(\([^\s()<>]+\)))*\))+(?:\(([^\s()<>]+|(\([^\s()<>]+\)))*\)|[^\s`!()\[\]{};:'\".,<>?«»“”‘’]))"#;
            let re = Regex::new(pattern).unwrap();
            if re.is_match(s) {
                Ok(())
            } else {
                Err(ValidationError::UrlError(s.clone()))
            }
        }
    }

    /// Validates that value is a valid IP address (or null)
    ///
    /// # Example
    ///
    /// ```
    /// use serde_json::json;
    /// use validate_ro::rules::Rule;
    ///
    /// let validator = Rule::ip();
    /// assert!(validator.validate(&json!("192.168.1.1")).is_ok());
    /// ```
    pub fn ip() -> impl Validator {
        move |value: &Value| {
            if value.is_null() {
                return Ok(())
            }
            let s = match value {
                Value::String(s) => s,
                _ => return Err(ValidationError::TypeError {
                    expected: "string".to_string(),
                    got: value.to_string(),
                }),
            };

            let re = Regex::new(r"^(\d{1,3})\.(\d{1,3})\.(\d{1,3})\.(\d{1,3})$").unwrap();
            if let Some(caps) = re.captures(s) {
                if caps.iter().skip(1).all(|m| m.unwrap().as_str().parse::<u8>().is_ok()) {
                    return Ok(());
                }
            }
            Err(ValidationError::IpError(s.clone()))
        }
    }

    /// Validates file extension against allowed set
    ///
    /// # Arguments
    ///
    /// * `allowed` - List of allowed extensions (without dots)
    ///
    /// # Example
    ///
    /// ```
    /// use serde_json::json;
    /// use validate_ro::rules::Rule;
    ///
    /// let validator = Rule::extensions(vec!["png".into(), "jpg".into()]);
    /// assert!(validator.validate(&json!("image.png")).is_ok());
    /// ```
    pub fn extensions(allowed: Vec<String>) -> impl Validator {
        ExtensionValidator {
            allowed: allowed.into_iter().collect(),
        }
    }

    /// Creates custom validator from closure
    ///
    /// # Arguments
    ///
    /// * `validator` - Validation function
    ///
    /// # Example
    ///
    /// ```
    /// use serde_json::json;
    /// use validate_ro::error::ValidationError;
    /// use validate_ro::rules::Rule;
    ///
    /// let validator = Rule::custom(|value| {
    ///     if value == "secret" {
    ///         Ok(())
    ///     } else {
    ///         Err(ValidationError::Custom("Invalid value".into()))
    ///     }
    /// });
    /// ```
    pub fn custom<F>(validator: F) -> impl Validator
    where
        F: Fn(&Value) -> ValidationResult+Send+Sync+ 'static
    {
        validator
    }

    /// Validates field value is unique in MongoDB collection
    ///
    /// # Arguments
    ///
    /// * `collection` - MongoDB collection name
    /// * `field` - Field name to check uniqueness
    /// * `exclude` - Optional document ID to exclude from check (for updates)
    ///
    /// # Example
    ///
    /// ```rust
    /// use validate_ro::rules::Rule;
    ///
    /// // For new documents:
    /// let validator = Rule::unique("users", "email", None);
    ///
    /// // When updating document:
    /// let validator = Rule::unique("users", "email", Some(user_id));
    /// ```
    pub fn unique(collection: &str, field: &str,exclude:Option<ObjectId>) -> impl Validator {
        UniqueValidator::new(collection, field,exclude)
    }
}

struct UniqueValidator {
    collection: String,
    field: String,
    current_id: Option<ObjectId>,
}

impl UniqueValidator {
    pub fn new(collection: &str, field: &str,exclude:Option<ObjectId>) -> Self {
        Self {
            collection: collection.to_string(),
            field: field.to_string(),
            current_id: exclude,
        }
    }
}

#[async_trait]
impl Validator for UniqueValidator {
    fn validate(&self, value: &Value) -> ValidationResult {
        // This is a placeholder - actual async validation needs to happen in validate_async
        if value.is_null() {
            return Ok(());
        }
        Err(ValidationError::Custom("Async validation required".to_string()))
    }

    async fn validate_async(&self, db: &Database, value: &Value) -> ValidationResult {
        if value.is_null() {
            return Ok(());
        }

        let collection: Collection<Document> = db.collection(&self.collection);
        let field_value = match value {
            Value::String(s) => Bson::String(s.clone()),
            Value::Number(n) if n.is_i64() => Bson::Int64(n.as_i64().unwrap()),
            Value::Number(n) if n.is_f64() => Bson::Double(n.as_f64().unwrap()),
            _ => return Err(ValidationError::TypeError {
                expected: "string or number".to_string(),
                got: value.to_string(),
            }),
        };

        let mut filter = doc! { &self.field: field_value };

        if let Some(current_id) = &self.current_id {
                filter.insert("_id", doc! { "$ne": current_id });
        }

        match collection.count_documents(filter).await {
            Ok(count) if count > 0 => {
                Err(ValidationError::UniqueError)
            }
            Ok(_) => Ok(()),
            Err(_) => {
                Err(ValidationError::Custom("Database error".to_string()))
            }
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
struct ExtensionValidator {
    allowed: HashSet<String>,
}

impl Validator for ExtensionValidator {
    fn validate(&self, value: &Value) -> ValidationResult {
        if value.is_null() {
            return Ok(())
        }
        let s = match value {
            Value::String(s) => s,
            _ => return Err(ValidationError::TypeError {
                expected: "string".to_string(),
                got: value.to_string(),
            }),
        };

        if let Some(ext) = s.split('.').last() {
            if self.allowed.contains(ext) {
                return Ok(());
            }
        }
        Err(ValidationError::ExtensionError(
            self.allowed.iter().cloned().collect(),
        ))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}


struct RegexValidator {
    pattern: Regex,
    message: Option<String>,
}

impl Validator for RegexValidator {
    fn validate(&self, value: &Value) -> ValidationResult {
        if value.is_null() {
            return Ok(())
        }
        let s = match value {
            Value::String(s) => s,
            _ => return Err(ValidationError::TypeError {
                expected: "string".to_string(),
                got: value.to_string(),
            }),
        };

        if self.pattern.is_match(s) {
            Ok(())
        } else {
            Err(ValidationError::RegexError(
                self.message.as_ref().map_or(s.clone(), |m| m.clone()),
            ))
        }
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


struct NotInValidator {
    values: Vec<Value>,
}

impl Validator for NotInValidator {
    fn validate(&self, value: &Value) -> ValidationResult {
        if value.is_null() {
            return Ok(())
        }
        if !self.values.contains(value) {
            return Ok(())
        }
        Err(ValidationError::NotInError(format!("{:?}", self.values)))
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


struct InValidator {
    values: Vec<Value>,
}

impl Validator for InValidator {
    fn validate(&self, value: &Value) -> ValidationResult {
        if value.is_null() {
            return Ok(())
        }
        if self.values.contains(value) {
            return Ok(())
        }
        Err(ValidationError::InError(format!("{:?}", self.values)))
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct EmailValidator {
    allowed_domains: Option<HashSet<String>>,
}

impl Validator for EmailValidator {
    fn validate(&self, value: &Value) -> ValidationResult {
        if value.is_null() {
            return Ok(())
        }
        let email = match value {
            Value::String(s) => s,
            _ => return Err(ValidationError::TypeError {
                expected: "string".to_string(),
                got: value.to_string(),
            }),
        };

        let parts: Vec<&str> = email.split('@').collect();
        if parts.len() != 2 {
            return Err(ValidationError::EmailError(email.clone()));
        }

        let name = parts[0];
        let domain = parts[1];
        let domain_parts: Vec<&str> = domain.split('.').collect();

        if domain_parts.len() < 2 {
            return Err(ValidationError::EmailError(email.clone()));
        }

        if domain_parts[1].len() < 2 {
            return Err(ValidationError::EmailError(email.clone()));
        }

        if let Some(allowed) = &self.allowed_domains {
            if !allowed.contains(domain) {
                return Err(ValidationError::EmailDomainError(domain.to_string()));
            }
        }

        if name.len() < 3 {
            return Err(ValidationError::EmailError(email.clone()));
        }

        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}



struct MaxValueValidator {
    max: f64,
}

impl Validator for MaxValueValidator {
    fn validate(&self, value: &Value) -> ValidationResult {
        if value.is_null() {
            return Ok(())
        }
        let num = match value {
            Value::Number(n) => n.as_f64().ok_or(ValidationError::TypeError {
                expected: "number".to_string(),
                got: value.to_string(),
            })?,
            _ => return Err(ValidationError::TypeError {
                expected: "number".to_string(),
                got: value.to_string(),
            }),
        };

        if num <= self.max {
            Ok(())
        } else {
            Err(ValidationError::MaxValueError {
                expected: self.max,
                got: num,
            })
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}


struct MinValueValidator {
    min: f64,
}

impl Validator for MinValueValidator {
    fn validate(&self, value: &Value) -> ValidationResult {
        if value.is_null() {
            return Ok(())
        }
        let num = match value {
            Value::Number(n) => n.as_f64().ok_or(ValidationError::TypeError {
                expected: "number".to_string(),
                got: value.to_string(),
            })?,
            _ => return Err(ValidationError::TypeError {
                expected: "number".to_string(),
                got: value.to_string(),
            }),
        };

        if num >= self.min {
            Ok(())
        } else {
            Err(ValidationError::MinValueError {
                expected: self.min,
                got: num,
            })
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct EqualValidator {
    value: Value,
}

impl Validator for EqualValidator {
    fn validate(&self, value: &Value) -> ValidationResult {
        if value.is_null() {
            return Ok(())
        }
        if value == &self.value {
            Ok(())
        } else {
            Err(ValidationError::EqualError {
                expected: self.value.to_string(),
                got: value.to_string(),
            })
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct MaxLengthValidator {
    max: usize,
}

impl Validator for MaxLengthValidator {
    fn validate(&self, value: &Value) -> ValidationResult {
        if value.is_null() {
            return Ok(())
        }
        let len = match value {
            Value::String(s) => s.len(),
            Value::Array(a) => a.len(),
            Value::Object(o) => o.len(),
            _ => return Err(ValidationError::TypeError {
                expected: "string, array, or object".to_string(),
                got: value.to_string(),
            }),
        };

        if len <= self.max {
            Ok(())
        } else {
            Err(ValidationError::MaxLengthError {
                expected: self.max,
                got: len,
            })
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}


struct MinLengthValidator {
    min: usize,
}

impl Validator for MinLengthValidator {
    fn validate(&self, value: &Value) -> ValidationResult {
        if value.is_null() {
            return Ok(())
        }
        let len = match value {
            Value::String(s) => s.len(),
            Value::Array(a) => a.len(),
            Value::Object(o) => o.len(),
            _ => return Err(ValidationError::TypeError {
                expected: "string, array, or object".to_string(),
                got: value.to_string(),
            }),
        };

        if len >= self.min {
            Ok(())
        } else {
            Err(ValidationError::MinLengthError {
                expected: self.min,
                got: len,
            })
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}


struct LengthValidator {
    length: usize,
}

impl Validator for LengthValidator {
    fn validate(&self, value: &Value) -> ValidationResult {
        if value.is_null() {
            return Ok(())
        }
        let len = match value {
            Value::String(s) => s.len(),
            Value::Array(a) => a.len(),
            Value::Object(o) => o.len(),
            _ => return Err(ValidationError::TypeError {
                expected: "string, array, or object".to_string(),
                got: value.to_string(),
            }),
        };

        if len == self.length {
            Ok(())
        } else {
            Err(ValidationError::LengthError {
                expected: self.length,
                got: len,
            })
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}