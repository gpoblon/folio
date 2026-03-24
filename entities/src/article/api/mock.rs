//! Mock implementation of the ArticleStore trait.
//! Useful for testing and development.
//! Do not use in production.
//! Simply call [`ArticleStore::mock()`] instead of building from real markdown files.

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::{
    article::model::{Article, ArticleMetadata},
    metadata::*,
};

impl super::model::ArticleStore {
    pub fn mock() -> Self {
        // A short lorem ipsum block to clone into every article's content
        const LOREM: &str = r#"Lorem ipsum dolor sit amet, consectetur adipiscing elit.
Sed non risus. Suspendisse lectus tortor, dignissim sit amet, adipiscing nec, ultricies sed, dolor.

Cras elementum ultrices diam. Maecenas ligula massa, varius a, semper congue, euismod non, mi.

    And this is a short article used to test our backend:

        tEST décalage.

---
created: 2026-03-05 14:52:38
modified: 2026-03-05 15:21:24
title: This is quite a long title
description: And this is quite a short description ... or maybe not, who knows. Do you? I won't know. I could go with lorem ipsum but it isn't as funny, is it..?
lang: en
tags: [ok, ko, ahah hah ... bon ;; '' "" ^^^ ]
state: published
expertise: UNDEFINED
---

> [!NOTE] pwovj wer
>
> qwpeofjweprg

> [!CAUTION] pwovj wer
> qwpeofjweprg

> [!TIP] pwovj wer
> qwpeofjweprg

> [!WARNING] pwovj wer
> qwpeofjweprg

> [!IMPORTANT] pwovj wer
> qwpeofjweprg



This is a short article used to test our backend:

1. Code block
```
code block
```
*italic* and **bold**

# A title

## A titl2

### A title 3

#### A title 4

##### A title 5

###### Title 6

A table

| Element                                                                         | Markdown Syntax                                     |
| ------------------------------------------------------------------------------- | --------------------------------------------------- |
| [Heading](https://www.markdownguide.org/basic-syntax/#headings)                 | `# H1   ## H2   ### H3`                             |
| [Bold](https://www.markdownguide.org/basic-syntax/#bold)                        | `**bold text**`                                     |
| [Italic](https://www.markdownguide.org/basic-syntax/#italic)                    | `*italicized text*`                                 |
| [Blockquote](https://www.markdownguide.org/basic-syntax/#blockquotes-1)         | `> blockquote`                                      |
| [Ordered List](https://www.markdownguide.org/basic-syntax/#ordered-lists)       | `1. First item   2. Second item   3. Third item   ` |
| [Unordered List](https://www.markdownguide.org/basic-syntax/#unordered-lists)   | `- First item   - Second item   - Third item   `    |
| [Code](https://www.markdownguide.org/basic-syntax/#code)                        | `` `code` ``                                        |
| [Horizontal Rule](https://www.markdownguide.org/basic-syntax/#horizontal-rules) | `---`                                               |
| [Link](https://www.markdownguide.org/basic-syntax/#links)                       | `[title](https://www.example.com)`                  |
| [Image](https://www.markdownguide.org/basic-syntax/#images-1)                   | `![alt text](image.jpg)`                            |
|                                                                                 |                                                     |
A rule

---


A ???
---
qwed

A link: [title](https://www.example.com)

✅ R

An image but it does not exist : ![alt text](/assets/test.png)

An image: ![/assets/test.png]

An image obsidian style: ![[/assets/test.png]]

A list:
- d
- d
- d
- fg
- g
- gh
    - ./
	- df
	    - ffg
	- rg
	- rg
	    - i rgq fewr f
- e e

1. dd
	. .
	. wwwww
		. qwef2wf


			 we fwerfg
2. wef wfw er

> What is this? A quote
> posc werfgre ge gertg


Sentence with code `Here's a sentence with a footnote. [^1]      [^1]: This is the footnote.`

Here's a sentence with a footnote. [^1]

term   : definition [^2]

~~STRIKETHROUGH The world is flat.~~

- [x] Task list
- [ ] Update the website
- [ ] Contact the media`

Emoji! :joy:

==Very important Words==


X^2

Harder:

<span style="color:blue">INLINE HTML some *blue* text</span>...

Hardest one:

<details>
<summary>Click me : Collapsible</summary>

### Heading
1. Foo
2. Bar
* Baz
* Qux
### Some Javascript
```js
function logSomething(something) {
console.log('Something', something);
}
```
</details>

[^1]: This is the footnote.
[^2]: This is another footnote.
"#;

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
                slug: "it/dev/lang/rust/intro.md",
                title: "Getting Started with Rust",
                description: "An introductory overview of Rust and its ecosystem.",
                tags: &["rust", "programming", "systems"],
                lang: kernel::lang::Lang::English,
                created_at: Some("2023-05-17 00:05:12"),
                modified_at: Some("2023-06-01 12:00:00"),
            },
            Seed {
                slug: "it/dev/lang/rust/ownership.md",
                title: "Understanding Ownership in Rust",
                description: "A focused look at ownership, borrowing and lifetimes.",
                tags: &["rust", "ownership", "memory"],
                lang: kernel::lang::Lang::English,
                created_at: Some("2023-04-02 09:15:00"),
                modified_at: None,
            },
            Seed {
                slug: "programming/web/backend/concurrency.md",
                title: "Concurrency Patterns for Backend Systems",
                description: "Common concurrency patterns and how Rust helps you implement them safely.",
                tags: &["programming", "concurrency", "rust"],
                lang: kernel::lang::Lang::English,
                created_at: Some("2023-03-20 14:30:45"),
                modified_at: Some("2023-05-01 08:00:00"),
            },
            Seed {
                slug: "programming/tools/debugging.md",
                title: "Practical Debugging Techniques",
                description: "Debugging strategies across languages and runtimes.",
                tags: &["programming", "debugging", "tools"],
                lang: kernel::lang::Lang::English,
                created_at: None,
                modified_at: None,
            },
            Seed {
                slug: "science/psychology/cognition.md",
                title: "Basics of Human Cognition",
                description: "An overview of cognitive processes and models.",
                tags: &["psychology", "cognition", "science"],
                lang: kernel::lang::Lang::English,
                created_at: Some("2022-12-10 07:45:30"),
                modified_at: Some("2023-01-02 10:00:00"),
            },
            Seed {
                slug: "science/psychology/pathology/autism.md",
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
                slug: "it/dev/lang/rust/performance.md",
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
                slug: seed.slug.to_string(),
                lang: seed.lang,
                tags: seed.tags.iter().map(|s| (*s).into()).collect(),
                state: State::Published,
                expertise: Expertise::Knowledgeable,
                created: seed.created_at.and_then(|s| {
                    chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S").ok()
                }),
                modified: seed.modified_at.and_then(|s| {
                    chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S").ok()
                }),
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
