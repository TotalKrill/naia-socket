extern "C" {
    pub fn naia_now() -> f64;
}

/// A Timestamp for a moment in time that can be read/written to/from a byte
/// stream
#[derive(Copy, Clone, PartialEq)]
pub struct Timestamp {
    time: u64,
}

impl Timestamp {
    /// Get a Timestamp for the current moment
    pub fn now() -> Self {
        unsafe {
            Timestamp {
                time: naia_now() as u64,
            }
        }
    }
}
