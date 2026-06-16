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
- umami: used for privacy-focused analytics. Cookies-free, completely anonymous.

## 🧭 Roadmap

See: [github project page](https://github.com/gpoblon/folio/projects/1).

## 📜 Usage & Rules

### Feature-sliced architecture (FSD)

This project applies a slightly adapted [FSD](https://feature-sliced.design).

It relies on the following technical *layers*:  
- *apps* - the package you'll run. Routing, entrypoint, providers. Glue & config only.
- *pages* - one crate per individual, plain page (1:1 app route). Composition through UI. No logic (mostly).
- *widgets* - self-contained structural UI blocks. Composition: connects features.
- *features* - user actions (e.g. add_to_cart). Heavy logic. Must be **verbs**. What is the user doing?
- *entities* - business data, used across project (e.g. user or product). Must be **nouns**. What is this object?
- *components* - reusable, pure UI components. No business logic.
- *kernel* - core functionalities reused across the upper layers (e.g. i18n). No business logic.

*pages* *widgets* *features* *entities* modules consist of *slices*. They divide their layer by domain.

Each slice holds technical segments: 
- *ui* - UI components. As pure as possible.
- *model* - Data types ; Hooks.
- *api* - Server functions ; queries.
On rare occasions, you may have: *config*.

### Rules

- A module can only import modules from lower layers (no siblings, no parents).
- Segments: only create segments that are useful for a given slice.
- Do not create new layers.
- Do not create new segments.
- No technical division in segments (e.g. no `hooks.rs`)
  If a segment gets too big: divide such as each file reflects the domain it represents.
- *kernel* & *components* are a last resort, they only contains pieces you could use in a totally different app.
- All translations go into `/assets/lang`
- All style goes into `app/assets/`
- All logic is colocated with UI. One exception: `/kernel` contains only logic, in such case ui goes into `/features`.

### General guidelines

- Keep coupling as low as possible, and cohesion as high as possible:
  - Law of Demeter: import only what is strictly required (`b.method()` rather than `a.b.method()`).
  - coupling means how much modules interact with each others (code wise).
    Some kind of coupling are worst than others: none > message > strict data > nested data > control meddling > external data > global data > data internal control
  - cohesion is related to the SRP principle: put together domain-related things that perform exactly 1 **action** (e.g. `register` or `place_order`).
- YAGNI: don't write code for something you don't need right now.
  Cover only the minimum requirment for the task, but do it well.
- KISS: the simpler, the better.
- Don't expose internals: encapsulate.
- Spend the time naming things properly

### Specific instructions

- Keep `pub` as strict as possible: no `pub` > `pub(super)` > `pub(crate)` > `pub`.
- Keep injection (hooks) only for the controller (parent) component. Pass props to children to keep it pure and decoupled.

## 📦 Setup / Deploy

### Run locally

Locally to iterate on the web version, using [dx](https://dioxuslabs.com/learn/0.7/getting_started/):

Setup:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs
export CARGO_HOME="$CARGO_HOME"
rustup default nightly
rustup target add wasm32-unknown-unknown
curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.s | bash
cargo binstall dioxus-cli@0.7.3 wasm-bindgen-cli@0.2.114
```

```sh
dx serve -p web --features=mock
```
Doing so will automatically:
1. Host the website locally (http://localhost:8080)
2. Generate tailwind css classes
3. Automatically reload updated website and style on code changes.

### Production deployment

To deploy a new release (production) version to https://gpoblon.net, ask a maintainer to run the following manual workflow: _CD (production)_.

### Staging deployment

To deploy a new staging (dev) version to https://gpoblon.net, ask a maintainer to run the following manual workflow: _CD (staging)_.

## 🤝 Hygiene

Please always perform the following checks before committing:  
1. ⚙️ `cargo build --workspace --all --all-features --tests`
2. 🧼 `cargo fmt --all`
3. 🩺 `cargo clippy --workspace --all --all-features --tests -- -D warnings`
4. 🧪 `cargo test --all-targets --all-features --workspace`

## 📄 License

The source code is published for **reference and portfolio purposes** under
[CC BY-NC-ND 4.0](https://creativecommons.org/licenses/by-nc-nd/4.0/).

You are welcome to read and learn from this code.
You may **not** copy, redistribute, or adapt it to build your own portfolio or
product without explicit written permission from the author.

Editorial content (articles, images, personal data) is © Gaëtan Poblon —
all rights reserved.

See [NOTICE](./NOTICE) and [LICENSE](./LICENSE).
