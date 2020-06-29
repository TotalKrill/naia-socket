use std::error::Error;
use std::fmt;
use std::net::SocketAddr;

/// An Error type specifically related to the Naia Server Socket
/// This is under construction and needs to be cleaned up
#[derive(Debug)]
pub enum NaiaServerSocketError {
    /// A wrapped error from another library/codebase
    Wrapped(Box<dyn Error>),
    /// An error indicating an inability to send to the given address
    SendError(SocketAddr),
}

impl fmt::Display for NaiaServerSocketError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            NaiaServerSocketError::Wrapped(boxed_err) => fmt::Display::fmt(boxed_err.as_ref(), f),
            NaiaServerSocketError::SendError(addr) => fmt::Display::fmt(&addr, f),
        }
    }
}

impl Error for NaiaServerSocketError {}
