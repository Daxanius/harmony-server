#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
#[non_exhaustive]
pub enum ServerError {
    Database(String),
    CouldNotCreate(String),
    HashFailed,
    AlreadyExists,
    NotFound,
    ExecutionFailed,
    FileNotFound,
}

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
#[non_exhaustive]
pub enum AuthError {
    InvalidPassword,
    UserNotFound,
    HashFailed,
    Database(String),
}
