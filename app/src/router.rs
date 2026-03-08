use dioxus::prelude::*;
use pages::{Article, Connect, Experience, Home, Knowledge, Project, Projects, TermsOfUse};
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
                // TODO: Implement a loading spinner or a more sophisticated loading indicator.
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
    #[route("/projects")]
    Projects {},
    #[route("/projects/:slug")]
    Project { slug: String },
    #[route("/experience")]
    Experience {},
    #[route("/knowledge")]
    Knowledge {},
    #[route("/connect")]
    Connect {},
    #[route("/terms-of-use")]
    TermsOfUse {},
    /// Dynamically defined routes based on fetched article slug
    #[route("/articles/:..slug")]
    Article { slug: Vec<String> },
}

impl Navigable for Route {
    const ITEMS: &[Self] = &[
        Route::Projects {},
        Route::Experience {},
        Route::Knowledge {},
        Route::Connect {},
    ];

    fn color(&self) -> &'static str {
        match self {
            Route::Projects {} | Route::Project { .. } => "projects",
            Route::Experience {} => "experience",
            Route::Knowledge {} => "knowledge",
            Route::Connect {} => "connect",
            _ => "primary",
        }
    }
    fn slug(&self) -> &'static str {
        match self {
            Route::Projects {} | Route::Project { .. } => "projects",
            Route::Experience {} => "experience",
            Route::Knowledge {} => "knowledge",
            Route::Connect {} => "connect",
            _ => "",
        }
    }
    fn icon(&self) -> Element {
        use components::svg;
        match self {
            Route::Projects {} | Route::Project { .. } => svg::Projects(),
            Route::Knowledge {} => svg::Knowledge(),
            Route::Experience {} => svg::Experience(),
            Route::Connect {} => svg::Connect(),
            _ => rsx! {},
        }
    }
}
