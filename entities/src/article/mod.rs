mod api;
pub mod model;
mod ui;

#[cfg(feature = "mock")]
mod mock;

pub use ui::Article;
