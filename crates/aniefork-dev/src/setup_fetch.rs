//! GridCraft's one way to download: HTTPS, for a setup ([`crate::setup`]) and nothing else.
//!
//! ureq over rustls, with Mozilla's roots built in, so that no system library is linked. It
//! follows no redirect itself: [`crate::setup::download`] does, to the hosts the question named
//! alone. Nothing decompresses a response: what is checked is the bytes as they came. A proxy the
//! person set in their environment (`HTTPS_PROXY`) is used, as ureq does.

use std::time::Duration;

use crate::setup::{Reply, Transport};

/// HTTPS, as a setup downloads.
pub struct Https {
    agent: ureq::Agent,
}

impl Default for Https {
    fn default() -> Self {
        Self::new()
    }
}

impl Https {
    pub fn new() -> Self {
        let config = ureq::Agent::config_builder()
            .https_only(true)
            .max_redirects(0)
            .http_status_as_error(false)
            .timeout_connect(Some(Duration::from_secs(30)))
            .timeout_recv_response(Some(Duration::from_secs(60)))
            // Freedoom's 24 MB on a slow line is the longest a setup takes.
            .timeout_recv_body(Some(Duration::from_secs(30 * 60)))
            .user_agent(format!("DevCraft/{} (GridCraft's C Development Kit, setting a game up)", crate::DEVCRAFT_VERSION))
            .build();
        Https { agent: config.into() }
    }
}

impl Transport for Https {
    fn get(&self, url: &str) -> Result<Reply, String> {
        let response = self.agent.get(url).call().map_err(|why| why.to_string())?;
        let status = response.status().as_u16();
        let location = response.headers().get("location").and_then(|value| value.to_str().ok()).map(str::to_string);
        let length = response.body().content_length();
        let body = response.into_body().into_reader();
        Ok(Reply { status, location, length, body: Box::new(body) })
    }
}
