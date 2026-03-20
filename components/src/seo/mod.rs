//! SEO component and supporting utilities.
//!
//! The [`Seo`] component renders all `<head>` metadata (Open Graph, Twitter
//! Cards, JSON-LD, hreflang, RSS discovery) from a single [`SeoProps`] value.

mod author;
mod breadcrumb;
mod component;
mod props;

pub use component::Seo;
pub use props::SeoProps;
