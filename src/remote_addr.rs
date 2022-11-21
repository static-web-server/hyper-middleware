//! Module representing the remote (peer) address of a connection.

use hyper::server::conn::AddrStream;
use std::net::SocketAddr;

/// Defines a method to get the remote (peer) address of a connection.
///
/// This trait might be needed to be implemented by for example custom TLS implementations.
pub trait RemoteAddr {
    /// Returns the remote (peer) address of this connection.
    fn remote_addr(&self) -> Option<SocketAddr>;
}

impl RemoteAddr for AddrStream {
    fn remote_addr(&self) -> Option<SocketAddr> {
        Some(self.remote_addr())
    }
}
