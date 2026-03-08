use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum Expertise {
    Novice,
    Knowledgeable,
    Expert,
    #[default]
    #[serde(other)]
    Undefined,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum State {
    Draft,
    Review,
    Published,
    Private,
    Archived,
    #[default]
    #[serde(other)]
    Undefined,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, strum::Display)]
#[strum(serialize_all = "kebab-case")]
#[serde(untagged)]
pub enum Intent {
    HandsOn,
    Concept,
    DeepDive,
    Review,
    CaseStudy,
    Reference,
    Essay,
    #[strum(default)]
    Other(String),
}

impl From<&str> for Intent {
    fn from(value: &str) -> Self {
        match value {
            "hands-on" => Intent::HandsOn,
            "concept" => Intent::Concept,
            "deep-dive" => Intent::DeepDive,
            "review" => Intent::Review,
            "case-study" => Intent::CaseStudy,
            "reference" => Intent::Reference,
            "essay" => Intent::Essay,
            _ => Intent::Other(value.to_string()),
        }
    }
}
