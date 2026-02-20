//! Mock implementation of the ArticleStore trait.
//! Useful for testing and development.
//! Do not use in production.
//! Simply call [`ArticleStore::mock()`] instead of

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::article::model::{Article, ArticleMetadata, ArticleStore, Expertise, State};

impl ArticleStore {
    pub fn mock() -> Self {
        // A short lorem ipsum block to clone into every article's content
        const LOREM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed non risus. Suspendisse lectus tortor, dignissim sit amet, adipiscing nec, ultricies sed, dolor. Cras elementum ultrices diam. Maecenas ligula massa, varius a, semper congue, euismod non, mi.";

        let mut articles = HashMap::new();

        // Use a small typed seed struct to make entries explicit and less error-prone
        struct Seed<'a> {
            slug: &'a str,
            title: &'a str,
            description: &'a str,
            tags: &'a [&'a str],
            lang: kernel::lang::Lang,
            created_at: Option<&'a str>,
            modified_at: Option<&'a str>,
        }

        // Typed seed slice (with optional created/modified date strings)
        let seeds: &[Seed] = &[
            Seed {
                slug: "/it/dev/lang/rust/intro.md",
                title: "Getting Started with Rust",
                description: "An introductory overview of Rust and its ecosystem.",
                tags: &["rust", "programming", "systems"],
                lang: kernel::lang::Lang::English,
                created_at: Some("2023-05-17 00:05:12"),
                modified_at: Some("2023-06-01 12:00:00"),
            },
            Seed {
                slug: "/it/dev/lang/rust/ownership.md",
                title: "Understanding Ownership in Rust",
                description: "A focused look at ownership, borrowing and lifetimes.",
                tags: &["rust", "ownership", "memory"],
                lang: kernel::lang::Lang::English,
                created_at: Some("2023-04-02 09:15:00"),
                modified_at: None,
            },
            Seed {
                slug: "/programming/web/backend/concurrency.md",
                title: "Concurrency Patterns for Backend Systems",
                description: "Common concurrency patterns and how Rust helps you implement them safely.",
                tags: &["programming", "concurrency", "rust"],
                lang: kernel::lang::Lang::English,
                created_at: Some("2023-03-20 14:30:45"),
                modified_at: Some("2023-05-01 08:00:00"),
            },
            Seed {
                slug: "/programming/tools/debugging.md",
                title: "Practical Debugging Techniques",
                description: "Debugging strategies across languages and runtimes.",
                tags: &["programming", "debugging", "tools"],
                lang: kernel::lang::Lang::English,
                created_at: None,
                modified_at: None,
            },
            Seed {
                slug: "/science/psychology/cognition.md",
                title: "Basics of Human Cognition",
                description: "An overview of cognitive processes and models.",
                tags: &["psychology", "cognition", "science"],
                lang: kernel::lang::Lang::English,
                created_at: Some("2022-12-10 07:45:30"),
                modified_at: Some("2023-01-02 10:00:00"),
            },
            Seed {
                slug: "/science/psychology/pathology/autism.md",
                title: "Understanding Autism Spectrum",
                description: "Key concepts and contemporary perspectives on autism.",
                tags: &["psychology", "autism", "pathology"],
                lang: kernel::lang::Lang::English,
                created_at: Some("2021-11-11 11:11:11"),
                modified_at: None,
            },
            Seed {
                slug: "/science/psychology/behavioral.md",
                title: "Behavioral Insights for Developers",
                description: "How behavioral psychology informs better product and developer decisions.",
                tags: &["psychology", "behavioral", "ux"],
                lang: kernel::lang::Lang::English,
                created_at: Some("2023-02-14 16:20:00"),
                modified_at: Some("2023-03-01 09:00:00"),
            },
            Seed {
                slug: "/it/dev/lang/rust/performance.md",
                title: "Rust for High Performance",
                description: "Techniques for squeezing performance out of Rust applications.",
                tags: &["rust", "performance", "systems"],
                lang: kernel::lang::Lang::English,
                created_at: Some("2023-06-10 22:05:05"),
                modified_at: Some("2023-06-15 18:30:00"),
            },
        ];

        for seed in seeds {
            let metadata = ArticleMetadata {
                title: seed.title.to_string(),
                description: seed.description.to_string(),
                lang: seed.lang,
                tags: seed.tags.iter().map(|s| s.to_string()).collect(),
                state: State::Published,
                expertise: Expertise::Knowedgeable,
                slug: seed.slug.to_string(),
                created: seed
                    .created_at
                    .and_then(|s| kernel::DateTime::try_from(s).ok()),
                modified: seed
                    .modified_at
                    .and_then(|s| kernel::DateTime::try_from(s).ok()),
            };

            let article = Article {
                metadata: metadata.clone(),
                content: LOREM.to_string(),
            };

            articles.insert(metadata.slug.clone(), article);
        }

        Self(Arc::new(RwLock::new(articles)))
    }
}
