use std::any::Any;

use async_trait::async_trait;
use mongodb::Database;
use serde_json::Value;
use crate::error::ValidationError;

#[async_trait]
pub trait Validator: Any+Send + Sync {
    fn validate(&self, value: &Value) -> ValidationResult;

    async fn validate_async(&self, _db: &Database, value: &Value) -> ValidationResult {
        self.validate(value)
    }

    fn as_any(&self) -> &dyn Any;
}


impl<F> Validator for F
where
    F: Fn(&Value) -> ValidationResult+Sync+Send+ 'static
{
    fn validate(&self, value: &Value) -> ValidationResult {
        self(value)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub type ValidationResult = Result<(), ValidationError>;