use dioxus::prelude::*;

/// Visual separator line (horizontal or vertical).
///
/// Renders a 1 px line coloured with `--color-muted-foreground` by default.
/// Callers can add Tailwind utility classes (spacing, colour overrides, etc.)
/// through the standard `class` attribute — they merge naturally with the
/// built-in defaults.
#[component]
pub fn Separator(
    /// Horizontal if true (default), vertical if false.
    #[props(default = true)]
    horizontal: bool,
    /// Mark as purely decorative (hides from assistive technology).
    #[props(default = false)]
    decorative: bool,
    /// Extra attributes forwarded to the underlying `<div>`.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let orientation = if horizontal { "horizontal" } else { "vertical" };
    let size = if horizontal {
        "w-full h-px"
    } else {
        "w-px h-full"
    };

    rsx! {
        div {
            role: if !decorative { "separator" } else { "none" },
            aria_orientation: if !decorative { orientation },
            "data-orientation": orientation,
            class: "bg-muted-foreground {size}",
            ..attributes,
            {children}
        }
    }
}
