use std::time::SystemTime;

/// A Timestamp for a moment in time that can be read/written to/from a byte
/// stream
#[derive(Copy, Clone, PartialEq)]
pub struct Timestamp {
    time: u64,
}

impl Timestamp {
    /// Get a Timestamp for the current moment
    pub fn now() -> Self {
        let time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("timing error!")
            .as_secs();
        Timestamp { time }
    }
}
