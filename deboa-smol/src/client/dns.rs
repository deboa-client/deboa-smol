//! DNS resolution for the Deboa HTTP client.
//!
//! This module provides DNS resolution functionality for the Deboa HTTP client.

use deboa::{
    dns::{DnsResolver, DnsResponse},
    errors::{DeboaError::Dns, DnsError},
    Result,
};
use rand::seq::SliceRandom;
use smol::net::resolve;
use std::net::IpAddr;

#[derive(Default, Clone)]
/// Default DNS resolver implementation using smol::net::resolve
pub struct DefaultDnsResolver;

impl DnsResolver for DefaultDnsResolver {
    async fn resolve(&self, host: String, port: u16) -> Result<DnsResponse> {
        let hostname = format!("{}:{}", host, port);
        let addrs = resolve(hostname).await;
        if let Err(e) = addrs {
            return Err(Dns(DnsError::Resolve { host, message: e.to_string() }));
        };

        let mut ips: Vec<IpAddr> = addrs
            .unwrap()
            .into_iter()
            .map(|addr| addr.ip())
            .collect();
        ips.shuffle(&mut rand::rng());

        Ok(DnsResponse::builder()
            .addresses(ips)
            .build())
    }
}
