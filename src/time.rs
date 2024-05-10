use std::time::Duration;

use wasm_timer::{SystemTime, UNIX_EPOCH};

/// Get the amount of time since the Unix epoch.
///
/// # Panics
///
/// Panics if the resulting time would somehow be negative.
#[must_use]
pub fn time_since_epoch() -> Duration {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
}
