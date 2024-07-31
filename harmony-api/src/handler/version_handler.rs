use rocket::get;

use crate::utils::environment;

#[get("/")]
pub fn get_version_handler() -> String {
    environment::get_version()
}
