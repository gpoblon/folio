use dioxus::prelude::*;
use dioxus_primitives::separator::{self, SeparatorProps};

#[component]
pub fn Separator(props: SeparatorProps) -> Element {
    // Split caller-supplied attributes: pull out `class` for the spacing
    // wrapper, forward everything else to the inner separator primitive.
    //
    // This keeps the visible line at exactly 1 px while still letting
    // callers control the surrounding space with e.g. `class: "py-4"`.
    let mut wrapper_class = String::new();
    let inner_attrs: Vec<Attribute> = props
        .attributes
        .into_iter()
        .filter(|attr| {
            if attr.name == "class" {
                if let dioxus::dioxus_core::AttributeValue::Text(ref v) = attr.value {
                    wrapper_class = v.clone();
                }
                false
            } else {
                true
            }
        })
        .collect();

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div {
            class: "{wrapper_class}",
            separator::Separator {
                class: "separator",
                horizontal: props.horizontal,
                decorative: props.decorative,
                attributes: inner_attrs,
                {props.children}
            }
        }
    }
}
