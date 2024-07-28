use crate::core::validation::result::ValidationError;
use regex::Regex;

#[derive(Debug)]
pub struct Username(String);

impl Username {
    pub fn new(name: &str) -> Result<Self, ValidationError> {
        let re = Regex::new(r"^[a-zA-Z0-9_]{3,16}$").unwrap();
        if !re.is_match(name) {
            Err(ValidationError::InvalidUserName(
                "Name must be 3-16 characters long and contain only letters, numbers, and underscores".to_string(),
            ))
        } else {
            Ok(Self(name.to_string()))
        }
    }

    #[must_use]
    pub fn value(&self) -> &String {
        &self.0
    }
}

#[derive(Debug)]
pub struct Password(String);

impl Password {
    pub fn new(password: &str) -> Result<Self, ValidationError> {
        if password.len() < 8 {
            Err(ValidationError::InvalidPassword(
                "Password must be at least 8 characters long".to_string(),
            ))
        } else if password.contains(char::is_whitespace) {
            Err(ValidationError::InvalidPassword(
                "Password must not contain whitespace".to_string(),
            ))
        } else if !Password::is_complex_enough(password) {
            Err(ValidationError::InvalidPassword(
                "Password must include letters, numbers, and special characters".to_string(),
            ))
        } else {
            Ok(Password(password.to_string()))
        }
    }

    fn is_complex_enough(password: &str) -> bool {
        let has_letter = password.chars().any(|c| c.is_alphabetic());
        let has_digit = password.chars().any(|c| c.is_numeric());
        let has_special = password
            .chars()
            .any(|c| !c.is_alphanumeric() && !c.is_whitespace());
        has_letter && has_digit && has_special
    }

    #[must_use]
    pub fn value(&self) -> &String {
        &self.0
    }
}

#[derive(Debug)]
pub struct Email(String);

impl Email {
    pub fn new(email: &str) -> Result<Self, ValidationError> {
        // Improved regex pattern for more accurate email validation
        let re = Regex::new(r"(^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$)").unwrap();
        if !re.is_match(email) {
            Err(ValidationError::InvalidEmail(
                "Invalid email format".to_string(),
            ))
        } else {
            Ok(Email(email.to_string()))
        }
    }

    #[must_use]
    pub fn value(&self) -> &String {
        &self.0
    }
}

#[derive(Debug)]
pub struct Name(String);

impl Name {
    pub fn new(name: &str) -> Result<Self, ValidationError> {
        let re = Regex::new(r"^[a-zA-Z0-9_ ]{3,32}$").unwrap();
        if !re.is_match(name) {
            return Err(ValidationError::InvalidName(
                "Name must be 3-32 characters long and contain only letters, numbers, underscores, and spaces".to_string(),
            ));
        }

        if name.trim() != name {
            return Err(ValidationError::InvalidName(
                "Name must not have leading or trailing spaces".to_string(),
            ));
        }

        Ok(Self(name.to_string()))
    }

    #[must_use]
    pub fn value(&self) -> &String {
        &self.0
    }
}

#[derive(Debug)]
pub struct FilePath(String);

impl FilePath {
    pub fn new(path: &str) -> Result<Self, ValidationError> {
        // Regex to validate a basic file path
        let re = Regex::new(r"^[a-zA-Z0-9_\-./\\]+$").unwrap();
        if !re.is_match(path) {
            Err(ValidationError::InvalidFilePath(
                "Invalid file path format".to_string(),
            ))
        } else {
            Ok(Self(path.to_string()))
        }
    }

    #[must_use]
    pub fn value(&self) -> &String {
        &self.0
    }
}

#[derive(Debug)]
pub struct YouTubeUrl(String);

impl YouTubeUrl {
    pub fn new(url: &str) -> Result<Self, ValidationError> {
        // Regex to validate and capture the YouTube video ID
        let re =
            Regex::new(r"^(https?://)?(www\.)?(youtube\.com/watch\?v=|youtu\.be/)(?P<id>[\w-]+)$")
                .unwrap();
        if let Some(captures) = re.captures(url) {
            if let Some(id) = captures.name("id") {
                let standardized_url = format!("https://youtu.be/{}", id.as_str());
                return Ok(Self(standardized_url));
            }
        }
        Err(ValidationError::InvalidYouTubeUrl(
            "Invalid YouTube URL format".to_string(),
        ))
    }

    #[must_use]
    pub fn value(&self) -> &String {
        &self.0
    }
}
