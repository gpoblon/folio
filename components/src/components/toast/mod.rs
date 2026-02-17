//! This module provides a high-level API for displaying toasts.
//!
//! Four variants: success, error, warning, info.
//!
//! Default duration: 5 seconds. Can be customized and made persistent.
//!
//! Note: this wrapper serves the sole purpose of providing a less verbose toast declaration.
//!
//! This toast API: `use_toast().success("title").send()` with optional method chaining: `use_toast().success("title").duration(5).description("desc").send()`
//! Dioxus toast API: `use_toast().success("title".to_string, ToastOption::new().duration(std::time::Duration::from_secs(2))).description("desc")`

mod component;
pub use component::ToastProvider;

use dioxus_primitives::toast::ToastOptions;

/// ToastDispatcher trait for sending toast notifications
///
/// # Example
/// ```
/// use dioxus::prelude::*;
/// use folio::components::toast::{ToastDispatcher, Toasts};
///
/// struct MyComponent {
///     toast: Toasts,
/// }
///
/// impl ToastDispatcher for MyComponent {
///     fn notify(&self) {
///         self.toast.success("Hello, world!").send();
///     }
/// }
/// ```
pub trait ToastDispatcher {
    /// Send a toast notification based on internal state (usually [`Signal`])
    fn notify(&self);
}

/// A high-level [`dioxus_primitives::toast::Toasts`] wrapper API
pub struct Toasts(dioxus_primitives::toast::Toasts);

impl<'a> Toasts {
    pub fn success(&'a self, message: impl Into<String>) -> ToastBuilder<'a> {
        ToastBuilder::new(&self.0, ToastKind::Success, message)
    }

    pub fn error(&'a self, message: impl Into<String>) -> ToastBuilder<'a> {
        ToastBuilder::new(&self.0, ToastKind::Error, message)
    }

    pub fn warning(&'a self, message: impl Into<String>) -> ToastBuilder<'a> {
        ToastBuilder::new(&self.0, ToastKind::Warning, message)
    }

    pub fn info(&'a self, message: impl Into<String>) -> ToastBuilder<'a> {
        ToastBuilder::new(&self.0, ToastKind::Info, message)
    }
}

enum ToastKind {
    Success,
    Error,
    Warning,
    Info,
}

pub struct ToastBuilder<'a> {
    provider: &'a dioxus_primitives::toast::Toasts,
    title: String,
    description: Option<String>,
    duration: Option<u64>,
    kind: ToastKind,
}

impl<'a> ToastBuilder<'a> {
    fn new(
        provider: &'a dioxus_primitives::toast::Toasts,
        kind: ToastKind,
        message: impl Into<String>,
    ) -> Self {
        Self {
            provider,
            kind,
            title: message.into(),
            description: None,
            duration: Some(5),
        }
    }

    /// Sets the duration of the toast in seconds.
    pub fn duration(mut self, seconds: u64) -> Self {
        self.duration = Some(seconds);
        self
    }

    /// Sets the toast to be permanent (until manually dismissed or window fully reloaded).
    pub fn permanent(mut self) -> Self {
        self.duration = None;
        self
    }

    /// Sets a description of the toast. Description is optional.
    pub fn description(mut self, details: impl Into<String>) -> Self {
        self.description = Some(details.into());
        self
    }

    /// Required call to dispatch the toast to the nearest [`ToastProvider`] component, rendering it immediately.
    ///
    /// This must be called under a [`ToastProvider`] component ([`ToastProvider`] could be a root component).
    pub fn send(self) {
        let options = {
            let mut options = ToastOptions::new();
            if let Some(description) = self.description {
                options = options.description(description);
            };
            match self.duration {
                Some(duration) => options.duration(std::time::Duration::from_secs(duration)),
                None => options.permanent(true),
            }
        };

        match self.kind {
            ToastKind::Success => {
                self.provider.success(self.title, options);
            }
            ToastKind::Error => {
                self.provider.error(self.title, options);
            }
            ToastKind::Warning => {
                self.provider.warning(self.title, options);
            }
            ToastKind::Info => {
                self.provider.info(self.title, options);
            }
        }
    }
}

/// # use_toast
///
/// The `use_toast` hook provides access to the [`Toast`] api from the nearest [`ToastProvider`] which lets you
/// dispatch toasts from anywhere in your component tree.
///
/// This must be called under a [`ToastProvider`] component.
///
/// ## Example
/// ```rust
/// use components::toast::{ToastProvider, use_toast};
///
/// #[component]
/// fn ToastButton() -> Element {
///     let toast_api = use_toast();
///
///     rsx! {
///         button {
///             onclick: move |_| {
///                 toast_api.info("Custom Toast".to_string());
///             },
///             "Info (60s)"
///         }
///     }
/// }
/// ```
pub fn use_toast() -> Toasts {
    Toasts(dioxus_primitives::toast::use_toast())
}
