use kernel::lang;
use serde::Deserialize;

const EXPERIENCES_EN: &str = include_str!("../../../resources/experiences.en.toml");
const EXPERIENCES_FR: &str = include_str!("../../../resources/experiences.fr.toml");

#[derive(Deserialize, Clone)]
pub(super) struct Experiences {
    pub jobs: Vec<Experience>,
    pub education: Vec<Experience>,
}

impl Experiences {
    // TODO fetch from server
    pub(super) fn use_server_experiences() -> Self {
        let content: &'static str = match lang::use_lang() {
            lang::Lang::English => EXPERIENCES_EN,
            lang::Lang::French => EXPERIENCES_FR,
        };

        match kernel::toml::from_str(content) {
            Ok(jobs) => jobs,
            Err(err) => panic!("Failed to parse experiences: {}", err),
        }
    }
}

#[derive(Deserialize, Clone, PartialEq)]
pub(super) struct Experience {
    /// official title of the job/certificate
    pub title: String,
    /// company or institution name
    pub organization: String,
    pub start_date: String,
    pub end_date: String,
    /// where the job/certificate was obtained
    pub location: String,
    /// short description of the experience
    pub focus: String,
    /// short summary of all missions
    pub overview: Option<String>,
    /// list of missions/deliverables/achievements (recursive)
    #[serde(default)]
    pub achievements: Vec<Achievement>,
}

#[derive(Deserialize, Clone, PartialEq)]
pub(super) struct Achievement {
    /// summary of the achievement in ~2-5 words
    pub label: Option<String>,
    /// Main description of the achievement
    pub description: String,
    pub link: Option<String>,
    /// If the achievement contains multiple parts, use a nested structure
    #[serde(default)]
    pub sub: Vec<Self>,
}
