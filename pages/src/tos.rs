use dioxus::prelude::*;
use kernel::lang;

const TOS_EN: &str = include_str!("../assets/tos.en.md");
const TOS_FR: &str = include_str!("../assets/tos.fr.md");

#[component]
pub fn TermsOfUse() -> Element {
    let content: &'static str = match lang::use_lang() {
        lang::Lang::English => TOS_EN,
        lang::Lang::French => TOS_FR,
    };

    rsx! {
        components::Seo {
            title: "Terms of Use",
            description: "Terms of use and legal notices for gpoblon.net.",
            canonical_path: "/terms-of-use",
            robots: "noindex, follow",
        }
        components::Markdown {
            class: "prose p-32 max-w-5xl text-justify mx-auto",
            content
        }
    }
}
