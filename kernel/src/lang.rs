pub use dioxus_i18n::prelude::i18n;
use dioxus_i18n::prelude::*;
pub use dioxus_i18n::t;
pub use dioxus_i18n::unic_langid::LanguageIdentifier;
use strum::IntoEnumIterator;
#[derive(Default, Clone, Copy, PartialEq, Eq, strum::EnumIter)]
pub enum Lang {
    #[default]
    French,
    English,
}
impl Lang {
    pub fn code(&self) -> &'static str {
        match self {
            Lang::French => "fr-FR",
            Lang::English => "en-US",
        }
    }
    pub fn slug(&self) -> &'static str {
        match self {
            Lang::French => "FR",
            Lang::English => "EN",
        }
    }
    fn asset_content(&self) -> &'static str {
        match self {
            Lang::French => include_str!("../../assets/lang/fr-FR.ftl"),
            Lang::English => include_str!("../../assets/lang/en-US.ftl"),
        }
    }
}
impl std::fmt::Display for Lang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.slug())
    }
}
impl From<LanguageIdentifier> for Lang {
    fn from(lang: LanguageIdentifier) -> Self {
        match lang.to_string().as_str() {
            "fr-FR" => Lang::French,
            "en-US" => Lang::English,
            _ => Lang::default(),
        }
    }
}
pub fn init_i18n() {
    let lang_default = LanguageIdentifier::from_bytes(Lang::default().code().as_bytes())
        .expect("Invalid default Language code");
    let mut config = I18nConfig::new(lang_default);
    for lang in Lang::iter() {
        let identifier =
            LanguageIdentifier::from_bytes(lang.code().as_bytes()).expect("Invalid Language code");
        config = config.with_locale((identifier, lang.asset_content()));
    }
    use_init_i18n(|| config);
}
pub fn use_i18n() -> dioxus_i18n::prelude::I18n {
    dioxus_i18n::prelude::i18n()
}
