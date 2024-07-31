use dotenvy::dotenv;
use std::env;

fn init_environment() {
    dotenv().ok();
}

/// # Panics
/// When the environment variable is not set
#[must_use]
pub fn get_jwt_secret() -> String {
    init_environment();
    env::var("JWT_SECRET").expect("JWT_SECRET must be set.")
}

#[must_use]
pub fn get_token_time_valid() -> u64 {
    init_environment();
    env::var("JWT_TIME_VALID")
        .expect("JWT_TIME_VALID must be set.")
        .parse::<u64>()
        .expect("JWT_TIME_VALID must be a valid positive number")
}

#[must_use]
pub fn get_stream_lifetime() -> u64 {
    init_environment();
    env::var("STREAM_LIFETIME")
        .expect("STREAM_LIFETIME must be set.")
        .parse::<u64>()
        .expect("STREAM_LIFETIME must be a valid positive number")
}

#[must_use]
pub fn get_version() -> String {
    init_environment();
    env::var("CARGO_PKG_VERSION").expect("CARGO_PKG_VERSION is not set.")
}
