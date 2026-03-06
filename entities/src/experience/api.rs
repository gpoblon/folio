use super::model::Experiences;
use kernel::lang;
use std::sync::LazyLock;

static EXPERIENCES_EN: LazyLock<Experiences> = LazyLock::new(|| {
    serde_saphyr::from_str(include_str!("../../assets/experiences.en.yaml"))
        .expect("experiences.en.yaml is invalid")
});

static EXPERIENCES_FR: LazyLock<Experiences> = LazyLock::new(|| {
    serde_saphyr::from_str(include_str!("../../assets/experiences.fr.yaml"))
        .expect("experiences.fr.yaml is invalid")
});

pub fn experiences() -> &'static Experiences {
    match lang::use_lang() {
        lang::Lang::English => &EXPERIENCES_EN,
        lang::Lang::French => &EXPERIENCES_FR,
    }
}
