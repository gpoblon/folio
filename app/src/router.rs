use dioxus::prelude::*;
use pages::{Article, Blog, Connect, Experience, Home, Lab, Project, TermsOfUse};
use widgets::{
    footer::Footer,
    nav::{NavBar, Navigable},
};

#[component]
fn Layout() -> Element {
    rsx! {
        div { class: "min-h-screen flex flex-col",
            NavBar::<Route> {}
            SuspenseBoundary {
                fallback: |_| rsx! {
                    div { class: "block justify-center items-center h-full", "Loading..." }
                },
                main { class: "flex-1 flex flex-col", Outlet::<Route> {} }
            }
            Footer { tos_route: Route::TermsOfUse {}.into() }
        }
    }
}

#[derive(Debug, Clone, Routable, PartialEq, Eq)]
pub enum Route {
    #[layout(Layout)]
    #[route("/")]
    Home {},
    #[route("/lab")]
    Lab {},
    #[route("/lab/:slug")]
    Project { slug: String },
    #[route("/experience")]
    Experience {},
    #[route("/blog")]
    Blog {},
    #[route("/contact")]
    Connect {},
    #[route("/terms-of-use")]
    TermsOfUse {},
    #[route("/articles/:..slug")]
    Article { slug: Vec<String> },
}

impl Navigable for Route {
    const ITEMS: &[Self] = &[
        Route::Blog {},
        Route::Lab {},
        Route::Experience {},
        Route::Connect {},
    ];

    fn color(&self) -> &'static str {
        match self {
            Route::Blog {} | Route::Article { .. } => "knowledge",
            Route::Lab {} | Route::Project { .. } => "projects",
            Route::Experience {} => "experience",
            Route::Connect {} => "connect",
            _ => "foreground",
        }
    }
    fn slug(&self) -> &'static str {
        match self {
            Route::Blog {} | Route::Article { .. } => "knowledge",
            Route::Lab {} | Route::Project { .. } => "projects",
            Route::Experience {} => "experience",
            Route::Connect {} => "connect",
            _ => "",
        }
    }
    fn icon(&self) -> Element {
        use components::Icons;
        let icon = match self {
            Route::Blog {} | Route::Article { .. } => Icons::Newsstand,
            Route::Lab {} | Route::Project { .. } => Icons::Experiment,
            Route::Experience {} => Icons::Landscape,
            Route::Connect {} => Icons::Join,
            _ => return rsx! {},
        };
        rsx! { components::Icon { icon } }
    }
}
