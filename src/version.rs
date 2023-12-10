use crate::enet_linked_version;

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
    pub fn current() -> Version {
        let version = unsafe { enet_linked_version() };
        Version {
            major: ((version >> 16) & 0xFF) as u8,
            minor: ((version >> 8) & 0xFF) as u8,
            patch: (version & 0xFF) as u8,
        }
    }
}
