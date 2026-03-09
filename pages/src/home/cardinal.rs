use dioxus::prelude::*;
use kernel::lang::t;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Cardinal {
    Identity,
    Experience,
    Knowledge,
    Connect,
    Projects,
}

impl Cardinal {
    pub(super) const NAV: [Self; 4] = [
        Self::Experience,
        Self::Connect,
        Self::Knowledge,
        Self::Projects,
    ];

    pub(super) fn label(self) -> String {
        match self {
            Self::Identity => t!("me"),
            Self::Projects => t!("projects"),
            Self::Experience => t!("experience"),
            Self::Knowledge => t!("knowledge"),
            Self::Connect => t!("connect"),
        }
    }

    pub(super) fn description(self) -> String {
        match self {
            Self::Identity => t!("home_identity"),
            Self::Projects => t!("home_projects"),
            Self::Experience => t!("home_experience"),
            Self::Knowledge => t!("home_knowledge"),
            Self::Connect => t!("home_contact"),
        }
    }

    pub(super) const fn route(self) -> &'static str {
        match self {
            Self::Identity => "/",
            Self::Projects => "/projects",
            Self::Experience => "/experience",
            Self::Knowledge => "/knowledge",
            Self::Connect => "/connect",
        }
    }

    pub(super) const fn color(self) -> &'static str {
        match self {
            Self::Identity => "foreground",
            Self::Projects => "projects",
            Self::Experience => "experience",
            Self::Knowledge => "knowledge",
            Self::Connect => "connect",
        }
    }

    /// Required to define the grid layout
    pub(super) const fn area(self) -> &'static str {
        match self {
            Self::Identity => "center",
            Self::Experience => "E",
            Self::Knowledge => "K",
            Self::Connect => "C",
            Self::Projects => "P",
        }
    }
}

#[component]
pub(super) fn CardinalCell(
    cardinal: Cardinal,
    is_active: bool,
    onhover: EventHandler<Option<Cardinal>>,
    onclick: EventHandler<Cardinal>,
) -> Element {
    let color = cardinal.color();
    let area = cardinal.area();

    rsx! {
        div {
            class: "flex items-center justify-center cursor-pointer z-20
                    py-2 px-20 sm:justify-self-stretch sm:self-stretch
                    cardinal-{area}",
            style: "grid-area:{area}",
            onmouseenter: move |_| onhover.call(Some(cardinal)),
            onmouseleave: move |_| onhover.call(None),
            onclick: move |_| onclick.call(cardinal),

            span {
                class: "whitespace-nowrap font-mono font-medium uppercase
                        text-{color} text-md sm:text-xl
                        border-2 border-transparent py-1.5 px-2
                        transition-all duration-300 ease-in-out",
                class: if is_active { "opacity-100 font-semibold scale-120 !border-current" } else { "opacity-70" },
                "{cardinal.label()}"
            }
        }
    }
}
