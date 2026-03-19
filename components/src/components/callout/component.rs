use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[non_exhaustive]
pub enum CalloutVariant {
    #[default]
    Info,
    Tip,
    Experiment,
    Warning,
    Caution,
}

impl CalloutVariant {
    pub fn icon(self) -> crate::Icons {
        match self {
            CalloutVariant::Info => crate::Icons::Info,
            CalloutVariant::Tip => crate::Icons::Lightbulb,
            CalloutVariant::Experiment => crate::Icons::Experiment,
            CalloutVariant::Warning => crate::Icons::Warning,
            CalloutVariant::Caution => crate::Icons::ReleaseAlert,
        }
    }

    pub fn class(&self) -> &'static str {
        match self {
            CalloutVariant::Info => "info",
            CalloutVariant::Tip => "tip",
            CalloutVariant::Experiment => "experiment",
            CalloutVariant::Warning => "warning",
            CalloutVariant::Caution => "caution",
        }
    }
}

/// The props for the [`Callout`] component.
#[derive(Props, Clone, PartialEq)]
pub struct CalloutProps {
    /// The variant of the callout
    #[props(default)]
    pub variant: CalloutVariant,

    /// The title/heading of the callout
    pub title: String,

    /// Additional attributes to extend the callout element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children content of the callout
    pub children: Element,
}

#[component]
pub fn Callout(props: CalloutProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        div {
            class: "callout",
            "data-variant": props.variant.class(),
            ..props.attributes,
            div {
                class: "callout-header",
                crate::Icon {
                    class: "callout-icon",
                    icon: props.variant.icon(),
                }
                h4 { class: "callout-title", "{props.title}" }
            }
            div {
                class: "callout-content",
                {props.children}
            }
        }
    }
}
