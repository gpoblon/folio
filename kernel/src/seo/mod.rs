//! Shared SEO constants and utilities available on **all** targets.
//!
//! Route handlers that need server-only dependencies (axum, reqwest…) live in
//! `routes` behind `#[cfg(feature = "server")]`.  Everything here is
//! pure data — no platform dependency — so it can be imported freely by
//! `components::Seo` and any other crate in the workspace.

mod keywords;

#[cfg(feature = "server")]
pub mod routes;

pub use keywords::Keywords;

#[cfg(feature = "server")]
pub use routes::{STATIC_SITEMAP_ROUTES, robots_txt, umami_api_proxy, umami_script_proxy};

// ── Site identity ─────────────────────────────────────────────────────────────────

pub const SITE_URL: &str = "https://gpoblon.net";
pub const SITE_NAME: &str = "Gaetan POBLON — Software Engineer";
pub const SITE_DESCRIPTION: &str = "Gaetan POBLON — Fullstack Rust Software Engineer. Deep-dive articles on Rust, Dioxus, Axum, WebAssembly, software architecture, and AI.";

// ── Author ────────────────────────────────────────────────────────────────────────

pub const AUTHOR_NAME: &str = "Gaetan POBLON";
pub const AUTHOR_EMAIL: &str = "hello@gpoblon.net";
pub const AUTHOR_JOB_TITLE: &str = "Rust Software Engineer";
pub const AUTHOR_GITHUB: &str = "https://github.com/gpoblon";
pub const AUTHOR_LINKEDIN: &str = "https://linkedin.com/in/gpoblon";
pub const AUTHOR_IMAGE: &str = "https://gpoblon.net/og-default.png";

/// All `sameAs` profile URLs for the site author.
pub const AUTHOR_SAME_AS: &[&str] = &[AUTHOR_GITHUB, AUTHOR_LINKEDIN];

// ── Open Graph defaults ───────────────────────────────────────────────────────────

pub const DEFAULT_OG_IMAGE: &str = "https://gpoblon.net/og-default.png";
pub const OG_IMAGE_WIDTH: u32 = 1200;
pub const OG_IMAGE_HEIGHT: u32 = 630;
