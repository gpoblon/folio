use super::model::Experiences;
use kernel::lang;
use std::sync::LazyLock;

static EXPERIENCES_EN: LazyLock<Experiences> = LazyLock::new(|| {
    toml::from_str(include_str!("../../assets/experiences.en.toml"))
        .expect("experiences.en.toml is invalid")
});

static EXPERIENCES_FR: LazyLock<Experiences> = LazyLock::new(|| {
    toml::from_str(include_str!("../../assets/experiences.fr.toml"))
        .expect("experiences.fr.toml is invalid")
});

pub fn experiences() -> &'static Experiences {
    match lang::use_lang() {
        lang::Lang::English => &EXPERIENCES_EN,
        lang::Lang::French => &EXPERIENCES_FR,
    }
}
