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

impl Expertise {
    /// Short description of what each expertise level means, for legend display.
    pub fn description(&self, lang: kernel::lang::Lang) -> &'static str {
        match lang {
            kernel::lang::Lang::French => match self {
                Expertise::Novice => "Notions de base, première approche",
                Expertise::Knowledgeable => "Compréhension solide, usage régulier",
                Expertise::Expert => "Maîtrise approfondie, recul critique",
                Expertise::Undefined => "",
            },
            kernel::lang::Lang::English => match self {
                Expertise::Novice => "Basic notions, first approach",
                Expertise::Knowledgeable => "Solid understanding, regular use",
                Expertise::Expert => "Deep mastery, critical perspective",
                Expertise::Undefined => "",
            },
        }
    }
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

/// The editorial intent of a piece of content.
///
/// Serialised as a plain lowercase / kebab-case string in YAML front-matter.
/// Unknown values are preserved in the `Other` variant so nothing is silently
/// dropped.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Intent {
    /// Surface-level analysis — broad overview, introduces a topic.
    Concept,
    /// Complete, in-depth analysis — the full picture.
    DeepDive,
    /// Raw information, no narrative — a lookup reference.
    Reference,
    /// Personal perspective — opinionated, argumentative.
    Essay,
    /// Action-oriented — step-by-step, practical.
    Guide,
    /// Any tag not matched by the variants above.
    Other(String),
}

impl Intent {
    /// Serialisation key used in YAML front-matter.
    pub fn key(&self) -> &str {
        match self {
            Intent::Concept => "concept",
            Intent::DeepDive => "deep-dive",
            Intent::Reference => "reference",
            Intent::Essay => "essay",
            Intent::Guide => "guide",
            Intent::Other(s) => s.as_str(),
        }
    }

    /// Display label, localised to `lang`.
    ///
    /// - FR: French tag name
    /// - EN: English base name
    pub fn label(&self, lang: kernel::lang::Lang) -> &str {
        match lang {
            kernel::lang::Lang::French => match self {
                Intent::Concept => "concept",
                Intent::DeepDive => "dossier",
                Intent::Reference => "référence",
                Intent::Essay => "essai",
                Intent::Guide => "guide",
                Intent::Other(s) => s.as_str(),
            },
            kernel::lang::Lang::English => match self {
                Intent::Concept => "concept",
                Intent::DeepDive => "deep-dive",
                Intent::Reference => "reference",
                Intent::Essay => "essay",
                Intent::Guide => "guide",
                Intent::Other(s) => s.as_str(),
            },
        }
    }

    /// One-liner description of what this intent means, for the legend tooltip.
    pub fn description(&self, lang: kernel::lang::Lang) -> &'static str {
        match lang {
            kernel::lang::Lang::French => match self {
                Intent::Concept => "analyse de surface",
                Intent::DeepDive => "analyse complète",
                Intent::Reference => "information brute, sans récit",
                Intent::Essay => "ma perspective, opinion",
                Intent::Guide => "orienté action",
                Intent::Other(_) => "",
            },
            kernel::lang::Lang::English => match self {
                Intent::Concept => "surface analysis",
                Intent::DeepDive => "complete analysis",
                Intent::Reference => "information, no narrative",
                Intent::Essay => "my perspective, opinionated",
                Intent::Guide => "action oriented",
                Intent::Other(_) => "",
            },
        }
    }

    /// Returns all known (non-`Other`) variants, useful for rendering the legend.
    pub fn known_variants() -> &'static [Intent] {
        &[
            Intent::Concept,
            Intent::DeepDive,
            Intent::Reference,
            Intent::Essay,
            Intent::Guide,
        ]
    }
}

impl std::fmt::Display for Intent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.key())
    }
}

impl From<&str> for Intent {
    fn from(value: &str) -> Self {
        match value {
            "concept" => Intent::Concept,
            "deep-dive" => Intent::DeepDive,
            "reference" => Intent::Reference,
            "essay" => Intent::Essay,
            "guide" => Intent::Guide,
            _ => Intent::Other(value.to_string()),
        }
    }
}
