use rocket::tokio::sync::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::time;

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

pub async fn stream_cleanup_task(state: StreamState) {
    let mut interval = time::interval(Duration::from_secs(1));

    loop {
        interval.tick().await;

        let mut file_data_map = state.file_data.write().await;
        let now = Instant::now();
        file_data_map.retain(|_, (_, expiration)| *expiration > now);
    }
}
