//! Mock implementation of the ProjectStore.
//! Useful for testing and development.
//! Simply call [`ProjectStore::mock()`] instead of building from YAML + GitHub READMEs.

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::project::model::{Project, ProjectStore};

const MOCK_README: &str = r#"# Mock Project

This is a **mock README** used during development.

## Features

- Feature one
- Feature two
- Feature three

## Getting Started

```bash
cargo run
```

## Architecture

| Layer | Description |
|-------|-------------|
| App | Entry point |
| Pages | Route-level components |
| Widgets | Composed UI blocks |
| Features | Business logic |
| Entities | Domain models |
| Components | Reusable UI primitives |
| Kernel | Core utilities |

## License

MIT
"#;

impl ProjectStore {
    /// For testing purposes only
    /// Mock to avoid fetching from GitHub.
    pub fn mock() -> Self {
        let project_metadata = super::model::load_all_project_metadata();

        let mut projects = HashMap::new();

        for meta in project_metadata {
            let slug = meta.repository.slug.clone();
            let project = Project {
                metadata: meta,
                content: MOCK_README.to_string(),
            };

            projects.insert(slug, project);
        }

        Self(Arc::new(RwLock::new(projects)))
    }
}
