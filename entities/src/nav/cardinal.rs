use dioxus::prelude::*;
use kernel::lang::t;

/// The five conceptual directions of the home-page compass.
///
/// `Identity` is the hub (center); the other four are interactive
/// cardinals rendered around it.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cardinal {
    Identity,
    Experience,
    Connect,
    Knowledge,
    Projects,
}

impl Cardinal {
    /// The four navigable cardinals (excludes `Identity`).
    pub const NAV: [Self; 4] = [
        Self::Knowledge,
        Self::Projects,
        Self::Experience,
        Self::Connect,
    ];

    // ── Identity ────────────────────────────────────────────────────────

    /// i18n display label.
    pub fn label(self) -> String {
        match self {
            Self::Identity => t!("me"),
            Self::Projects => t!("projects"),
            Self::Experience => t!("experience"),
            Self::Knowledge => t!("knowledge"),
            Self::Connect => t!("connect"),
        }
    }

    /// i18n long description shown in the center hub.
    pub fn description(self) -> String {
        match self {
            Self::Identity => t!("home_identity"),
            Self::Projects => t!("home_projects"),
            Self::Experience => t!("home_experience"),
            Self::Knowledge => t!("home_knowledge"),
            Self::Connect => t!("home_contact"),
        }
    }

    /// Optional subtitle beneath the cardinal label (e.g. availability).
    pub fn subtitle(self) -> Option<String> {
        match self {
            Self::Connect => Some(t!("home_availability")),
            _ => None,
        }
    }

    // ── Routing & layout ────────────────────────────────────────────────

    /// Target URL for navigation on click.
    pub const fn route(self) -> &'static str {
        match self {
            Self::Identity => "/",
            Self::Projects => "/lab",
            Self::Experience => "/experience",
            Self::Knowledge => "/blog",
            Self::Connect => "/contact",
        }
    }

    /// CSS grid-area name used by `.home-grid`.
    pub const fn area(self) -> &'static str {
        match self {
            Self::Identity => "center",
            Self::Experience => "E",
            Self::Knowledge => "K",
            Self::Connect => "C",
            Self::Projects => "P",
        }
    }

    // ── Presentation ────────────────────────────────────────────────────

    /// Tailwind color token suffix.
    pub const fn color(self) -> &'static str {
        match self {
            Self::Identity => "foreground",
            Self::Projects => "projects",
            Self::Experience => "experience",
            Self::Knowledge => "knowledge",
            Self::Connect => "connect",
        }
    }

    /// Icon element for each cardinal direction.
    pub fn icon(self) -> Element {
        let icon = match self {
            Self::Experience => components::Icons::Landscape,
            Self::Knowledge => components::Icons::Newsstand,
            Self::Connect => components::Icons::Join,
            Self::Projects => components::Icons::Experiment,
            Self::Identity => return rsx! {},
        };
        rsx! { components::Icon { class: "text-3xl", icon } }
    }
}
