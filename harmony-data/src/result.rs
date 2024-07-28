#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
#[non_exhaustive]
pub enum Error {
    NotFound,
    AlreadyExists,
    Database(String),
}
