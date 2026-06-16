#[allow(clippy::useless_format)]
pub mod build_info;
pub mod lang;
pub mod seo;
pub mod theme;
pub mod umami;

#[cfg(feature = "server")]
pub mod mail;

#[cfg(feature = "server")]
pub mod config;

#[cfg(feature = "server")]
pub mod git;

#[cfg(feature = "server")]
pub mod resources;

#[cfg(feature = "server")]
pub mod rate_limit;

pub use timeflow::prelude::DateTime;
