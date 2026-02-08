use dioxus::prelude::*;
use kernel::lang;
use std::cell::LazyCell;

#[component]
pub fn TermsOfUse() -> Element {
    let content: LazyCell<&str> = LazyCell::new(|| match lang::use_lang() {
        lang::Lang::English => include_str!("../../resources/tos.en.md"),
        lang::Lang::French => include_str!("../../resources/tos.fr.md"),
    });

    rsx! {
        components::Markdown {
            content: *content
        }
    }
}
