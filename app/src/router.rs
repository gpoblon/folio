use dioxus::prelude::*;
use pages::{Connect, Experience, Home, Knowledge, Projects, TermsOfUse};
use widgets::{
    footer::Footer,
    nav::{NavBar, Navigable},
};

#[component]
fn Layout() -> Element {
    rsx! {
        div { class: "min-h-screen flex flex-col",
            NavBar::<Route> {}
            main { class: "container center-content pt-28 md:pt-32 pb-10 flex-1", Outlet::<Route> {} }
            Footer { tos_route: Route::TermsOfUse {}.into() }
        }
    }
}

#[derive(Debug, Clone, Copy, Routable, PartialEq, Eq)]
pub enum Route {
    #[layout(Layout)]
    #[route("/")]
    Home {},
    #[route("/projects")]
    Projects {},
    #[route("/experience")]
    Experience {},
    #[route("/knowledge")]
    Knowledge {},
    #[route("/connect")]
    Connect {},
    #[route("/terms-of-use")]
    TermsOfUse {},
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
            Route::Projects {} => "projects",
            Route::Experience {} => "experience",
            Route::Knowledge {} => "knowledge",
            Route::Connect {} => "connect",
            _ => "primary",
        }
    }
    fn slug(&self) -> &'static str {
        match self {
            Route::Projects {} => "projects",
            Route::Experience {} => "experience",
            Route::Knowledge {} => "knowledge",
            Route::Connect {} => "connect",
            _ => "",
        }
    }
    fn icon(&self) -> Element {
        use components::svg;
        match self {
            Route::Projects {} => svg::Projects(),
            Route::Knowledge {} => svg::Knowledge(),
            Route::Experience {} => svg::Experience(),
            Route::Connect {} => svg::Connect(),
            _ => rsx! {},
        }
    }
}
