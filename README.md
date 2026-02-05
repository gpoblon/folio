[![ci](https://github.com/gpoblon/folio/actions/workflows/ci.yml/badge.svg)](https://github.com/gpoblon/folio/actions/workflows/ci.yml)

## 📦 Project: `folio`

##### ❓ What is it?

Well, it's my folio.

It tells people about me, my work, and my projects.
It could also serves as a playground for me to experiment.
It finally is a way for me to share and contribute.

## ✨ Features

- My work experience,
- Personal projects I have been or am working on. Be it open/closed/available source,
- Blog posts about software development, and other un/related topics,
- A way to contact me, and to share my contact information.

## 🧭 Roadmap

See: [github project page](https://github.com/gpoblon/folio/projects/1).

## 📦 Setup / Deploy

### Local usage

Locally to iterate on the web version, using [dx](https://dioxuslabs.com/learn/0.7/getting_started/):
```sh
dx serve -p web --hotpatch
```
Doing so will automatically:
1. Host the website locally (http://localhost:8080)
2. Generate tailwind css classes
3. Automatically reload updated website and style on code changes.

### Production deployment

To deploy a new release (production) version to https://folio.gpoblon.net, ask a maintainer to run the following manual workflow: _CD (production)_.

### Staging deployment

To deploy a new staging (dev) version to https://folio.dev.gpoblon.net, ask a maintainer to run the following manual workflow: _CD (staging)_.

## 🤝 Hygiene

Please always perform the following checks before committing:  
1. ⚙️ `cargo build --workspace --all --all-features --tests`
2. 🧼 `cargo fmt --all`
3. 🩺 `cargo clippy --workspace --all --all-features --tests -- -D warnings`
4. 🧪 `cargo test --all-targets --all-features --workspace`

## 📄 License - Proprietary

Copyright © 2026 Gaëtan Poblon.
All rights reserved.
Applies to both editorial content and source code.

See __NOTICE__.
