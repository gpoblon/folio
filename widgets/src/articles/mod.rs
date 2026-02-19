//! Blog / articles / knowledge base
//!
//! Content for this section is fetched from a github repository containing markdown files (obsidian vault).
//!
//! Articles have metadata, metadata format is parsed as follows:
//! ---
//! date: 2020-08-21 # yaml
//! ---
//!
//! Only articles in a `published` state are displayed
//!
//! TODO:
//! - Implement github api calls to fetch articles
//! - Render markdown content
//! - Implement search functionality
//! - Implement list of articles page
//!

mod api;
mod model;
mod ui;

pub use ui::ArticleGrid;
