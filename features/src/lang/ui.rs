use dioxus::prelude::*;
use strum::IntoEnumIterator;

#[component]
pub fn SelectLanguage() -> Element {
    let i18n = kernel::lang::use_i18n();
    let current_lang = kernel::lang::Lang::from(i18n.language());
    let mut selected_lang = use_signal(|| current_lang);

    rsx! {
        components::dropdown::DropdownMenu {
            default_open: false,
            components::dropdown::DropdownMenuTrigger {
                "{selected_lang().slug()}"
            }
            components::dropdown::DropdownMenuContent {
                for (idx, lang) in kernel::lang::Lang::iter().enumerate() {
                    components::dropdown::DropdownMenuItem {
                        value: lang,
                        index: idx,
                        disabled: selected_lang() == lang,
                        on_select: move |lang: kernel::lang::Lang| {
                            selected_lang.set(lang);
                            let lang_code = lang.code();
                            let mut i18n_local = i18n;
                            i18n_local.set_language(
                                kernel::lang::LanguageIdentifier::from_bytes(lang_code.as_bytes())
                                    .expect("Invalid Language code"),
                            );
                        },
                        "{lang.slug()}"
                    }
                }
            }
        }
    }
}
