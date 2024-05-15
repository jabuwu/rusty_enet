use core::time::Duration;

/// Get the amount of time since the Unix epoch, for use with
/// [`HostSettings::time`](`crate::HostSettings::time`).
///
/// # Panics
///
/// Panics if the resulting time would somehow be negative.
#[must_use]
pub fn time_since_epoch() -> Duration {
    #[cfg(not(target_arch = "wasm32"))]
    {
        #[cfg(not(feature = "std"))]
        {
            panic!("ENet host must be provided a custom time function in no_std environment.");
        }
        #[cfg(feature = "std")]
        {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("Time went backwards")
        }
    }
    #[cfg(target_arch = "wasm32")]
    {
        Duration::from_millis(js_sys::Date::now() as u64)
    }
}
