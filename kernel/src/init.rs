//! Initialization utilities for the kernel module.
//!
//! This module handles setting up global state required by various subsystems,
//! such as the Rustls crypto provider.

#[cfg(feature = "server")]
pub fn init_rustls() {
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
}
