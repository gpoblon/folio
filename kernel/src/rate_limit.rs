//! Simple in-memory rate limiter.
//!
//! Tracks request timestamps per key and rejects requests that exceed the
//! configured limit within a sliding time window.
//!
//! Stale entries are pruned on every call to [`RateLimiter::check`].
//!
//! ## Key strategy
//!
//! [`fingerprint_key`] builds a composite key from the client IP address and
//! the sender email for security reasons.
//!
//! ## IP extraction
//!
//! Client IP is resolved via [`axum_client_ip::ClientIp`], which reads from
//! the source configured at router startup (see [`ClientIpSource`]).
//!
//! | Deployment | Recommended source | How it works |
//! |---|---|---|
//! | Behind Nginx / reverse proxy | `ClientIpSource::XRealIp` | Reads `X-Real-IP`, set to `$remote_addr` by the proxy — unspoofable from outside |
//! | Direct / no proxy | `ClientIpSource::ConnectInfo` | Reads the raw TCP socket address — no header involved |
//!
//! **Proxy requirement (when using `XRealIp`):** Nginx must set
//! `proxy_set_header X-Real-IP $remote_addr` and strip any client-supplied
//! `X-Real-IP` header before it reaches the app.

pub use axum_client_ip::{ClientIp, ClientIpSource};

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// A clonable, thread-safe rate limiter backed by an in-memory sliding window.
///
/// Wrap in an Axum [`Extension`](axum::Extension) so every server function
/// that needs throttling can extract it.
#[derive(Clone)]
pub struct RateLimiter {
    inner: Arc<Mutex<HashMap<String, Vec<Instant>>>>,
    max_requests: u32,
    window: Duration,
}

impl RateLimiter {
    /// Create a new rate limiter.
    ///
    /// * `max_requests` – maximum number of allowed requests per key within
    ///   the sliding `window`.
    /// * `window` – the sliding time window.
    ///
    /// Example: `RateLimiter::new(3, Duration::from_secs(600))` allows
    /// **3 requests per 10 minutes** per key.
    pub fn new(max_requests: u32, window: Duration) -> Self {
        Self {
            inner: Arc::new(Mutex::new(HashMap::new())),
            max_requests,
            window,
        }
    }

    /// Record a request for `key` and return `Ok(())` if under the limit,
    /// or `Err(RateLimitError)` if the limit has been exceeded.
    ///
    /// Stale timestamps (older than the window) are pruned on every call.
    pub fn check(&self, key: &str) -> Result<(), RateLimitError> {
        let now = Instant::now();
        let cutoff = now - self.window;

        let mut map = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        let timestamps = map.entry(key.to_owned()).or_default();

        // Prune expired timestamps
        timestamps.retain(|&t| t > cutoff);

        if timestamps.len() as u32 >= self.max_requests {
            return Err(RateLimitError);
        }

        timestamps.push(now);
        Ok(())
    }
}

/// Returned when a rate limit has been exceeded.
#[derive(Debug, Clone, thiserror::Error)]
#[error("Rate limit exceeded. Please try again later.")]
pub struct RateLimitError;

/// Build a composite rate-limit key from the client IP and sender email.
///
/// Keying on IP alone would penalise shared-NAT users; keying on email alone
/// is insufficient for persistent abuse. The composite key addresses both.
pub fn fingerprint_key(client_ip: &ClientIp, email: &str) -> String {
    format!("{}:{}", client_ip.0, email)
}
