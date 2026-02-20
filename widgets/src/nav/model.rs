use dioxus::prelude::*;

/// Contract a `Route` enum must satisfy to be rendered by the nav widgets.
///
/// Mirrors how Dioxus itself uses `Router::<R: Routable>` — the upper layer
/// (app) implements this trait on its concrete `Route` enum, while the lower
/// layer (widgets) stays generic and FSD-compliant.
pub trait Navigable: Routable + PartialEq + Clone + 'static {
    const ITEMS: &[Self];
    /// Tailwind color token suffix for this route.
    fn color(&self) -> &'static str;
    /// i18n key / identifier slug.
    fn slug(&self) -> &'static str;
    /// SVG icon element for this item.
    fn icon(&self) -> Element;
}
