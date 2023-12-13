/// A version representing ENet's version schema.
///
/// Get the current version with [`Version::current`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
}

impl Version {
    /// Get the version of the ENet library.
    #[must_use]
    pub const fn current() -> Version {
        Version {
            major: 1,
            minor: 3,
            patch: 17,
        }
    }
}
