use serde::ser::{Serialize, Serializer, SerializeSeq};

#[derive(Debug)]
pub enum ValidationError {
    Required,
    TypeError { expected: String, got: String },
    LengthError { expected: usize, got: usize },
    MinLengthError { expected: usize, got: usize },
    MaxLengthError { expected: usize, got: usize },
    EqualError { expected: String, got: String },
    MinValueError { expected: f64, got: f64 },
    MaxValueError { expected: f64, got: f64 },
    NumericError(String),
    AcceptedError(String),
    EmailError(String),
    EmailDomainError(String),
    InError(String),
    NotInError(String),
    RegexError(String),
    UrlError(String),
    IpError(String),
    ExtensionError(Vec<String>),
    UniqueError,
    FileSizeError { min: u64, max: u64 },

    Custom(String),
}


impl Serialize for ValidationError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ValidationError::Required => {
                Ok(serializer.serialize_str("required_error")?)
            }
            ValidationError::TypeError { expected, got } => {
                let mut seq = serializer.serialize_seq(Some(2))?;
                seq.serialize_element("type_error")?;
                seq.serialize_element(&[expected,got])?;
                seq.end()
            }
            ValidationError::LengthError { expected, got } => {
                let mut seq = serializer.serialize_seq(Some(2))?;
                seq.serialize_element("len_error")?;
                seq.serialize_element(&[expected,got])?;
                seq.end()
            }
            ValidationError::MinLengthError { expected, got } => {
                let mut seq = serializer.serialize_seq(Some(2))?;
                seq.serialize_element("min_len_error")?;
                seq.serialize_element(&[expected,got])?;
                seq.end()
            }
            ValidationError::MaxLengthError { expected, got } => {
                let mut seq = serializer.serialize_seq(Some(2))?;
                seq.serialize_element("max_len_error")?;
                seq.serialize_element(&[expected,got])?;
                seq.end()
            }
            ValidationError::EqualError { expected, got } => {
                let mut seq = serializer.serialize_seq(Some(2))?;
                seq.serialize_element("eq_error")?;
                seq.serialize_element(&[expected,got])?;
                seq.end()
            }
            ValidationError::MinValueError { expected, got } => {
                let mut seq = serializer.serialize_seq(Some(2))?;
                seq.serialize_element("min_error")?;
                seq.serialize_element(&[expected,got])?;
                seq.end()
            }
            ValidationError::MaxValueError { expected, got } => {
                let mut seq = serializer.serialize_seq(Some(2))?;
                seq.serialize_element("max_error")?;
                seq.serialize_element(&[expected,got])?;
                seq.end()
            }
            ValidationError::NumericError(a) => {
                let mut seq = serializer.serialize_seq(Some(2))?;
                seq.serialize_element("numeric_error")?;
                seq.serialize_element(&[a])?;
                seq.end()
            }
            ValidationError::AcceptedError(a) => {
                let mut seq = serializer.serialize_seq(Some(2))?;
                seq.serialize_element("accepted_error")?;
                seq.serialize_element(&[a])?;
                seq.end()
            }
            ValidationError::EmailError(a) => {
                let mut seq = serializer.serialize_seq(Some(2))?;
                seq.serialize_element("email_error")?;
                seq.serialize_element(&[a])?;
                seq.end()
            }
            ValidationError::EmailDomainError(a) => {
                let mut seq = serializer.serialize_seq(Some(2))?;
                seq.serialize_element("email_domain_name_error")?;
                seq.serialize_element(&[a])?;
                seq.end()
            }
            ValidationError::InError(a) => {
                let mut seq = serializer.serialize_seq(Some(2))?;
                seq.serialize_element("in_error")?;
                seq.serialize_element(&[a])?;
                seq.end()
            }
            ValidationError::NotInError(a) => {
                let mut seq = serializer.serialize_seq(Some(2))?;
                seq.serialize_element("not_in_error")?;
                seq.serialize_element(&[a])?;
                seq.end()
            }
            ValidationError::RegexError(a) => {
                let mut seq = serializer.serialize_seq(Some(2))?;
                seq.serialize_element("regex_error")?;
                seq.serialize_element(&[a])?;
                seq.end()
            }
            ValidationError::UrlError(a) => {
                let mut seq = serializer.serialize_seq(Some(2))?;
                seq.serialize_element("url_error")?;
                seq.serialize_element(&[a])?;
                seq.end()
            }
            ValidationError::IpError(a) => {
                let mut seq = serializer.serialize_seq(Some(2))?;
                seq.serialize_element("ip_error")?;
                seq.serialize_element(&[a])?;
                seq.end()
            }
            ValidationError::ExtensionError(a) => {
                let mut seq = serializer.serialize_seq(Some(2))?;
                seq.serialize_element("extension_error")?;
                seq.serialize_element(&[a])?;
                seq.end()
            }
            ValidationError::UniqueError => {
                Ok(serializer.serialize_str("unique_error")?)
            }
            ValidationError::FileSizeError { min, max } => {
                let mut seq = serializer.serialize_seq(Some(2))?;
                seq.serialize_element("file_size_error")?;
                seq.serialize_element(&[min,max])?;
                seq.end()
            }
            ValidationError::Custom(a) => {
                let mut seq = serializer.serialize_seq(Some(2))?;
                seq.serialize_element("validate_error")?;
                seq.serialize_element(&[a])?;
                seq.end()
            }
        }
    }
}