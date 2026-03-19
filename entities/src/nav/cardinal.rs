use kernel::lang::t;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cardinal {
    Identity,
    Experience,
    Knowledge,
    Connect,
    Projects,
}

impl Cardinal {
    pub const NAV: [Self; 4] = [
        Self::Experience,
        Self::Knowledge,
        Self::Connect,
        Self::Projects,
    ];

    pub fn label(self) -> String {
        match self {
            Self::Identity => t!("me"),
            Self::Projects => t!("projects"),
            Self::Experience => t!("experience"),
            Self::Knowledge => t!("knowledge"),
            Self::Connect => t!("connect"),
        }
    }

    pub fn description(self) -> String {
        match self {
            Self::Identity => t!("home_identity"),
            Self::Projects => t!("home_projects"),
            Self::Experience => t!("home_experience"),
            Self::Knowledge => t!("home_knowledge"),
            Self::Connect => t!("home_contact"),
        }
    }

    pub const fn route(self) -> &'static str {
        match self {
            Self::Identity => "/",
            Self::Projects => "/projects",
            Self::Experience => "/experience",
            Self::Knowledge => "/knowledge",
            Self::Connect => "/connect",
        }
    }

    pub const fn color(self) -> &'static str {
        match self {
            Self::Identity => "foreground",
            Self::Projects => "projects",
            Self::Experience => "experience",
            Self::Knowledge => "knowledge",
            Self::Connect => "connect",
        }
    }

    /// Required to define the grid layout
    pub const fn area(self) -> &'static str {
        match self {
            Self::Identity => "center",
            Self::Experience => "E",
            Self::Knowledge => "C",
            Self::Connect => "K",
            Self::Projects => "P",
        }
    }
}
