use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct Experiences {
    pub jobs: Vec<Experience>,
    pub education: Vec<Experience>,
}

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct Experience {
    /// Official title of the job/certificate.
    pub title: String,
    /// Company or institution name.
    pub organization: String,
    pub start_date: String,
    pub end_date: String,
    /// Where the job/certificate was obtained.
    pub location: String,
    /// Short description of the experience.
    pub focus: String,
    /// Short summary of all missions.
    pub overview: Option<String>,
    /// List of missions/deliverables/achievements (recursive).
    #[serde(default)]
    pub achievements: Vec<Achievement>,
}

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct Achievement {
    /// Summary of the achievement in ~2-5 words.
    pub label: Option<String>,
    /// Main description of the achievement.
    pub description: String,
    pub link: Option<String>,
    /// If the achievement contains multiple parts, use a nested structure.
    #[serde(default)]
    pub sub: Vec<Self>,
}
