//! Shared SEO constants available on **all** targets (server, web, desktop, mobile).
//!
//! Route handlers that need server-only dependencies (axum, reqwest…) live in
//! `seo_routes.rs` behind `#[cfg(feature = "server")]`.  Everything here is
//! pure data — no platform dependency — so it can be imported freely by
//! `components::Seo` and any other crate in the workspace.

// ── Site identity ─────────────────────────────────────────────────────────────

pub const SITE_URL: &str = "https://gpoblon.net";
pub const SITE_NAME: &str = "Gaetan POBLON — Software Engineer";
pub const SITE_DESCRIPTION: &str =
    "Fullstack Rust Software Engineer. Articles on Rust, Dioxus, software architecture, and AI.";

// ── Author ────────────────────────────────────────────────────────────────────

pub const AUTHOR_NAME: &str = "Gaetan POBLON";
pub const AUTHOR_EMAIL: &str = "hello@gpoblon.net";
pub const AUTHOR_JOB_TITLE: &str = "Rust Software Engineer";
pub const AUTHOR_GITHUB: &str = "https://github.com/gpoblon";
pub const AUTHOR_LINKEDIN: &str = "https://linkedin.com/in/gpoblon";

// ── Open Graph defaults ───────────────────────────────────────────────────────

pub const DEFAULT_OG_IMAGE: &str = "https://gpoblon.net/og-default.png";
pub const OG_IMAGE_WIDTH: u32 = 1200;
pub const OG_IMAGE_HEIGHT: u32 = 630;
