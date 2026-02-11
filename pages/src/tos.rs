use dioxus::prelude::*;
use kernel::lang;

const TOS_EN: &str = include_str!("../../resources/tos.en.md");
const TOS_FR: &str = include_str!("../../resources/tos.fr.md");

#[component]
pub fn TermsOfUse() -> Element {
    let content: &'static str = match lang::use_lang() {
        lang::Lang::English => TOS_EN,
        lang::Lang::French => TOS_FR,
    };

    rsx! {
        components::Markdown {
            class: "container center-content p-18",
            content
        }
    }
}
