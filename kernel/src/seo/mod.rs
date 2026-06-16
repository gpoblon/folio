//! Shared SEO constants and utilities available on **all** targets.
//!
//! Route handlers that need server-only dependencies (axum, reqwest…) live in
//! `routes` behind `#[cfg(feature = "server")]`.  Everything here is
//! pure data — no platform dependency — so it can be imported freely by
//! `components::Seo` and any other crate in the workspace.

mod keywords;

#[cfg(feature = "server")]
pub mod routes;

pub use keywords::Keywords;

#[cfg(feature = "server")]
pub use routes::{robots_txt, STATIC_SITEMAP_ROUTES};

// ── Site identity ─────────────────────────────────────────────────────────────────

pub const SITE_URL: &str = "https://gpoblon.net";
pub const SITE_NAME: &str = "Gaëtan POBLON | Rust Software Engineer";
pub const SITE_DESCRIPTION: &str = "Portfolio of Gaëtan POBLON, Rust Software Engineer in France. Discover my cross-functional & core experience, projects, and articles. Connect for opportunities.";

// ── Author ────────────────────────────────────────────────────────────────────────

pub const AUTHOR_NAME: &str = "Gaëtan POBLON";
pub const AUTHOR_GIVEN_NAME: &str = "Gaëtan";
pub const AUTHOR_FAMILY_NAME: &str = "POBLON";
pub const AUTHOR_EMAIL: &str = "gaetan@gpoblon.net";
pub const AUTHOR_JOB_TITLE: &str = "Rust Software Engineer";
pub const AUTHOR_GITHUB: &str = "https://github.com/gpoblon";
pub const AUTHOR_LINKEDIN: &str = "https://linkedin.com/in/gpoblon";
pub const AUTHOR_IMAGE: &str = "https://gpoblon.net/og-default.png";

/// A short bio optimized for LLM/GEO extraction.
pub const AUTHOR_BIO: &str = "Gaëtan POBLON is a Rust Software Engineer with years of cross-functional experience in tech, product, and operations. From heading education at 42 Angoulême — scaling the campus to 592 students — to engineering a declarative Infrastructure-as-Code compiler at Normation and architecting a 70 000 LoC fullstack Rust codebase (Axum, Dioxus, Tailwind), he translates complex business expectations into high-performance digital products. Based in France, he specializes in Rust, cross-platform development, software architecture, and product engineering.";

/// Key topics the author is an expert in, used for GEO entity association.
pub const AUTHOR_KNOWS_ABOUT: &[&str] = &[
    "Rust Programming Language",
    "Software Architecture",
    "Fullstack Development",
    "Cross-platform Development",
    "Dioxus Framework",
    "Axum Web Framework",
    "WebAssembly",
    "Systems Programming",
    "Backend Development",
    "Frontend Development",
    "Domain-Driven Design",
    "Compiler Design",
    "Open Source Software",
    "Infrastructure as Code",
    "Product Management",
    "Team Management",
    "Team Leadership",
    "Education Technology",
    "SurrealDB",
    "PostgreSQL",
    "Artificial Intelligence",
    "DevOps",
    "CI/CD",
    "Tailwind CSS",
    "Technical Writing",
];

/// All `sameAs` profile URLs for the site author.
pub const AUTHOR_SAME_AS: &[&str] = &[AUTHOR_GITHUB, AUTHOR_LINKEDIN];

// ── Author — Structured career & education (GEO entity enrichment) ────────────

/// Location for local/geo SEO signals.
pub const AUTHOR_LOCATION: &str = "Niort, Nouvelle-Aquitaine, France";

/// Alumni organizations for `alumniOf` JSON-LD.
/// Each tuple: (name, url, @type).
pub const AUTHOR_ALUMNI: &[(&str, &str, &str)] = &[
    ("42 Paris", "https://42.fr", "EducationalOrganization"),
    (
        "University of Burgundy",
        "https://en.u-bourgogne.fr",
        "CollegeOrUniversity",
    ),
];

/// Occupation history for `hasOccupation` / `OrganizationRole` JSON-LD.
/// Each tuple: (role_name, organization_name, start_date, end_date, description).
pub const AUTHOR_OCCUPATIONS: &[(&str, &str, &str, &str, &str)] = &[
    (
        "Head of Education & IT",
        "42 Angoulême",
        "2021-08",
        "2026-02",
        "Spearheaded campus launch scaling to 600 students and 30+ corporate partnerships. Led product ownership of a substantial fullstack Rust educational platform (Axum, Dioxus, Tailwind).",
    ),
    (
        "Software Engineer",
        "Normation (Rudder)",
        "2019-12",
        "2021-03",
        "A developer of rudder-lang, a declarative Infrastructure-as-Code language in Rust. Built compiler toolchain: lexer, parser, AST, semantic analysis, and transpiler.",
    ),
    (
        "Fullstack Web Developer",
        "uRehab",
        "2017-09",
        "2018-01",
        "Designed frontend interfaces in React, built API and conversational chatbot logic in Node.js, and modeled data with MongoDB.",
    ),
];

/// Credentials / degrees for `hasCredential` JSON-LD.
/// Each tuple: (credential_name, institution, year).
pub const AUTHOR_CREDENTIALS: &[(&str, &str, &str)] = &[
    (
        "Digital Technologies Architect (MSc equivalent)",
        "42 Paris",
        "2020",
    ),
    ("Bachelor of Private Law", "University of Burgundy", "2016"),
];

/// Hackathon awards for `award` JSON-LD.
pub const AUTHOR_AWARDS: &[&str] = &[
    "42Startup Hackathon Award",
    "SexTechLab Hackathon Award",
    "Société Générale Hackathon Award",
];

// ── Open Graph defaults ───────────────────────────────────────────────────────────

pub const DEFAULT_OG_IMAGE: &str = "https://gpoblon.net/og-default.png";
pub const OG_IMAGE_WIDTH: u32 = 1200;
pub const OG_IMAGE_HEIGHT: u32 = 630;
