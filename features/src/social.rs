use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct SocialLinkProps {
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    pub prefix: &'static str,
    pub label: &'static str,
    pub href: &'static str,
    pub alt: Option<&'static str>,
}

#[component]
pub fn SocialLink(props: SocialLinkProps) -> Element {
    rsx! {
        components::Separator {}
        a {
            class: "flex items-baseline w-full pl-8 hover:opacity-80 transition-opacity no-underline! link",
            href: props.href,
            target: if props.href.starts_with("mailto:") { "_self" } else { "_blank" },
            rel: "noopener noreferrer",
            span {
                class: "text-xs text-muted-foreground",
                {props.prefix}
            }
            span {
                class: "text-lg flex-grow text-left",
                {props.label}
            }
            components::Icon {
                alt: props.alt.unwrap_or(props.label),
                class: "text-xl sm:text-3xl",
                icon: components::Icons::ArrowOutward
            }
        }
    }
}
