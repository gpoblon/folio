use dioxus::{hooks::use_resource, logger::tracing};
use entities::article::model::{ArticleMetadata, Expertise, State};
use kernel::lang::t;

pub(super) fn use_resource_article_list_metadata() -> dioxus::hooks::Resource<Vec<ArticleMetadata>>
{
    use_resource(move || {
        let toast = components::toast::use_toast();

        async move {
            match super::api::articles().await {
                Ok(articles) => articles,
                Err(err) => {
                    tracing::error!("Failed to fetch articles metadata: {}", err);
                    toast
                        .error(t!("article_list_metadata_error"))
                        .description(err.to_string())
                        .send();
                    Vec::new()
                }
            }
        }
    })
}

pub(super) fn mock_resource_article_list_metadata() -> Vec<ArticleMetadata> {
    vec![
        ArticleMetadata {
            title: "Introduction to Rust".to_string(),
            description: "A brief introduction to the Rust programming language.".to_string(),
            lang: kernel::lang::Lang::English,
            tags: vec!["rust".to_string(), "programming".to_string()],
            state: State::Published,
            expertise: Expertise::Novice,
            slug: "/IT/dev/lang/rust/intro.md".to_string(),
            created_at: None,
            modified_at: None,
        },
        ArticleMetadata {
            title: "Understanding Autism".to_string(),
            description: "An overview of autism spectrum disorder.".to_string(),
            lang: kernel::lang::Lang::English,
            tags: vec!["science".to_string(), "psychology".to_string()],
            state: State::Published,
            expertise: Expertise::Knowedgeable,
            slug: "/science/psychology/pathology/autism.md".to_string(),
            created_at: None,
            modified_at: None,
        },
        ArticleMetadata {
            title: "Introduction to Python".to_string(),
            description: "A brief introduction to the Python programming language.".to_string(),
            lang: kernel::lang::Lang::English,
            tags: vec!["python".to_string(), "programming".to_string()],
            state: State::Published,
            expertise: Expertise::Novice,
            slug: "/IT/dev/lang/python/intro.md".to_string(),
            created_at: None,
            modified_at: None,
        },
        ArticleMetadata {
            title: "Introduction to JavaScript".to_string(),
            description: "A brief introduction to the JavaScript programming language.".to_string(),
            lang: kernel::lang::Lang::English,
            tags: vec!["javascript".to_string(), "programming".to_string()],
            state: State::Published,
            expertise: Expertise::Novice,
            slug: "/IT/dev/lang/javascript/intro.md".to_string(),
            created_at: None,
            modified_at: None,
        },
        ArticleMetadata {
            title: "Introduction to Java".to_string(),
            description: "A brief introduction to the Java programming language.".to_string(),
            lang: kernel::lang::Lang::English,
            tags: vec!["java".to_string(), "programming".to_string()],
            state: State::Published,
            expertise: Expertise::Novice,
            slug: "/IT/dev/lang/java/intro.md".to_string(),
            created_at: None,
            modified_at: None,
        },
    ]
}
