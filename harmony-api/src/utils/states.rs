use rocket::tokio::sync::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

// Since it is inefficient to read a file every time the user asks
// for a part of a filestream, we will stream files using a loaded state
#[derive(Debug, Clone, Default)]
pub struct StreamState {
    #[allow(clippy::type_complexity)]
    pub file_data: Arc<RwLock<HashMap<String, (Vec<u8>, Instant)>>>,
}

impl StreamState {
    #[must_use]
    pub fn new() -> Self {
        Self {
            file_data: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}
