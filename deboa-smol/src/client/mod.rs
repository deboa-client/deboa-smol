/// DNS resolution for the Deboa HTTP client.
///
/// This module provides DNS resolution functionality for the Deboa HTTP client.pub(crate) mod dns;
pub(crate) mod dns;
pub(crate) mod http;
pub(crate) mod tls;

pub use dns::DefaultDnsResolver;
pub use http::conn::DeboaConnection::*;
pub use http::conn::pool::HttpConnectionPool;