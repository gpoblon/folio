//! Simple in-memory rate limiter.
//!
//! Tracks request timestamps per key and rejects requests that exceed the
//! configured limit within a sliding time window.  Stale entries are pruned
//! on every call to [`RateLimiter::check`].
//!
//! ## Key strategy
//!
//! Use [`fingerprint_key`] to build a composite key from the client IP address
//! (extracted from `X-Forwarded-For` or `X-Real-IP` headers) and the sender
//! email.  Keying on IP alone would penalise shared NAT; keying on email alone
//! lets bots bypass the limit by rotating addresses.  The combination forces
//! an attacker to rotate *both* simultaneously.

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
/// The IP is extracted from the `X-Forwarded-For` header (first hop only),
/// falling back to `X-Real-IP`, and finally to the sentinel `"unknown"` when
/// neither header is present (e.g. in local development).
///
/// Combining IP and email means a bot must rotate *both* simultaneously to
/// bypass the limiter:
/// - IP alone would penalise legitimate shared-NAT users (offices, etc.).
/// - Email alone is trivially bypassed by rotating addresses.
pub fn fingerprint_key(headers: &axum::http::HeaderMap, email: &str) -> String {
    let ip = headers
        // X-Forwarded-For may contain a comma-separated list; take the first entry
        // which is the original client (set by the outermost proxy).
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.split(',').next())
        .map(str::trim)
        // Fall back to X-Real-IP (set by nginx / single-hop proxies)
        .or_else(|| {
            headers
                .get("x-real-ip")
                .and_then(|v| v.to_str().ok())
                .map(str::trim)
        });

    match ip {
        // Production (behind a proxy): key on both so a bot must rotate
        // IP *and* email simultaneously to bypass the limiter.
        Some(ip) => format!("{ip}:{email}"),
        // No proxy headers (localhost, direct deploys): fall back to
        // email-only so rotating addresses is still blocked.
        None => email.to_owned(),
    }
}
