use dioxus::prelude::*;
use strum::IntoEnumIterator;
#[component]
pub fn SelectLanguage() -> Element {
    let i18n = kernel::lang::use_i18n();
    let current_lang = kernel::lang::Lang::from(i18n.language());
    let mut selected_lang = use_signal(|| current_lang);
    let mut is_open = use_signal(|| false);
    rsx! {
        div { class: "relative inline-flex",
            button {
                class: "size-10 text-primary border border-primary bg-primary flex items-center justify-center",
                onclick: move |_| is_open.set(!is_open()),
                "{selected_lang().slug()}"
            }
            if is_open() {
                div { class: "absolute top-full left-0 mt-2 z-50 w-10 border border-primary bg-primary text-primary shadow-lg",
                    for lang in kernel::lang::Lang::iter() {
                        button {
                            class: "block w-full py-2 text-center hover:bg-accent hover:text-accent",
                            disabled: selected_lang() == lang,
                            onclick: move |_| {
                                selected_lang.set(lang);
                                is_open.set(false);
                                let lang_code = lang.code();
                                let mut i18n_local = i18n;
                                i18n_local
                                    .set_language(
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
}
