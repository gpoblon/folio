use crate::lang::Lang;

pub const BUILD_DATE_EN: &str = env!("BUILD_DATE_EN");
pub const BUILD_DATE_FR: &str = env!("BUILD_DATE_FR");

pub fn build_date(lang: Lang) -> &'static str {
    match lang {
        Lang::French => BUILD_DATE_FR,
        Lang::English => BUILD_DATE_EN,
    }
}
