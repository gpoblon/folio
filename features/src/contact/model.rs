use serde::{Deserialize, Serialize};

/// Raw contact form data as submitted by the user
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContactFormData {
    pub name: String,
    pub email: String,
    pub subject: String,
    pub message: String,
}

/// Validated contact form data ready for processing
#[derive(Debug, Clone)]
pub struct ValidatedContactForm {
    pub name: String,
    pub email: String,
    pub subject: String,
    pub message: String,
}

/// Validation error types for contact form
#[derive(Debug, Clone, PartialEq)]
pub enum ContactFormError {
    EmptyName,
    EmptyEmail,
    InvalidEmail,
    EmptySubject,
    EmptyMessage,
}

impl std::fmt::Display for ContactFormError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContactFormError::EmptyName => write!(f, "Name cannot be empty"),
            ContactFormError::EmptyEmail => write!(f, "Email cannot be empty"),
            ContactFormError::InvalidEmail => write!(f, "Invalid email format"),
            ContactFormError::EmptySubject => write!(f, "Subject cannot be empty"),
            ContactFormError::EmptyMessage => write!(f, "Message cannot be empty"),
        }
    }
}

impl std::error::Error for ContactFormError {}

impl TryFrom<ContactFormData> for ValidatedContactForm {
    type Error = ContactFormError;

    fn try_from(form: ContactFormData) -> Result<Self, Self::Error> {
        let name = form.name.trim().to_string();
        if name.is_empty() {
            return Err(ContactFormError::EmptyName);
        }

        let email = form.email.trim().to_string();
        if email.is_empty() {
            return Err(ContactFormError::EmptyEmail);
        }
        if !email.contains('@') || !email.contains('.') {
            return Err(ContactFormError::InvalidEmail);
        }

        let subject = form.subject.trim().to_string();
        if subject.is_empty() {
            return Err(ContactFormError::EmptySubject);
        }

        let message = form.message.trim().to_string();
        if message.is_empty() {
            return Err(ContactFormError::EmptyMessage);
        }

        Ok(ValidatedContactForm {
            name,
            email,
            subject,
            message,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_form_data() -> ContactFormData {
        ContactFormData {
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            subject: "Hello".to_string(),
            message: "This is a test message".to_string(),
        }
    }

    #[test]
    fn test_valid_form() {
        let form = valid_form_data();
        let result: Result<ValidatedContactForm, _> = form.try_into();
        assert!(result.is_ok());
    }

    #[test]
    fn test_empty_name() {
        let mut form = valid_form_data();
        form.name = "   ".to_string();
        let result: Result<ValidatedContactForm, _> = form.try_into();
        assert_eq!(result.unwrap_err(), ContactFormError::EmptyName);
    }

    #[test]
    fn test_empty_email() {
        let mut form = valid_form_data();
        form.email = "".to_string();
        let result: Result<ValidatedContactForm, _> = form.try_into();
        assert_eq!(result.unwrap_err(), ContactFormError::EmptyEmail);
    }

    #[test]
    fn test_invalid_email() {
        let mut form = valid_form_data();
        form.email = "invalid-email".to_string();
        let result: Result<ValidatedContactForm, _> = form.try_into();
        assert_eq!(result.unwrap_err(), ContactFormError::InvalidEmail);
    }

    #[test]
    fn test_empty_subject() {
        let mut form = valid_form_data();
        form.subject = "".to_string();
        let result: Result<ValidatedContactForm, _> = form.try_into();
        assert_eq!(result.unwrap_err(), ContactFormError::EmptySubject);
    }

    #[test]
    fn test_empty_message() {
        let mut form = valid_form_data();
        form.message = "   ".to_string();
        let result: Result<ValidatedContactForm, _> = form.try_into();
        assert_eq!(result.unwrap_err(), ContactFormError::EmptyMessage);
    }
}
