#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
#[non_exhaustive]
pub enum ValidationError {
    InvalidUserName(String),
    InvalidPassword(String),
    InvalidEmail(String),
    InvalidFilePath(String),
    InvalidYouTubeUrl(String),
    InvalidName(String),
}
