use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SearchBarProps {
    /// Two-way binding for the search text
    pub query: Signal<String>,
    /// "Dropdown" propositions, strictly computed by the parent
    pub suggestions: Vec<String>,
    /// Fired when a user explicitly clicks a dropdown proposition
    /// Used to trigger side effects
    pub on_select: Option<EventHandler<String>>,
    #[props(default = "Search...".to_string())]
    pub placeholder: String,
}

#[component]
pub fn SearchBar(mut props: SearchBarProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div { class: "search-container",
            input {
                class: "search-input",
                type: "text",
                placeholder: "{props.placeholder}",
                value: "{props.query.read()}",
                oninput: move |evt| props.query.set(evt.value())
            }

            // Internal Action: Displaying the propositions
            if !props.suggestions.is_empty() {
                ul { class: "search-dropdown",
                    for suggestion in props.suggestions {
                        li {
                            class: "search-suggestion data-[root=true]:font-semibold",
                            "data-root": suggestion.as_bytes().iter().filter(|&&b| b == b'/').take(2).count() == 1,
                            key: "{suggestion}",
                            // Trigger the external action upon selection
                            onmousedown: move |_| {
                                props.query.set(suggestion.clone());
                                if let Some(on_select) = &props.on_select {
                                    on_select.call(suggestion.clone());
                                }
                            },
                            "{suggestion}"
                        }
                    }
                }
            }
        }
    }
}
