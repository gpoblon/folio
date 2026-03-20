//! Build-time metadata derived from a single `BUILD_DATE` env var (`YYYY-MM-DD`).

use crate::lang::Lang;

/// ISO 8601 date set at build time — used directly as sitemap `<lastmod>`.
pub const BUILD_DATE: &str = env!("BUILD_DATE");

const MONTHS_EN: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

const MONTHS_FR: [&str; 12] = [
    "janvier",
    "février",
    "mars",
    "avril",
    "mai",
    "juin",
    "juillet",
    "août",
    "septembre",
    "octobre",
    "novembre",
    "décembre",
];

/// Returns the build date formatted for display in the given language.
///
/// - English: "1 January 2025"
/// - French:  "1 janvier 2025"
///
/// Panics in debug only if `BUILD_DATE` is malformed — a build-time invariant
/// that can never be violated in production.
pub fn build_date(lang: Lang) -> String {
    let (year, month, day) = parse_build_date(BUILD_DATE);
    let months = match lang {
        Lang::English => &MONTHS_EN,
        Lang::French => &MONTHS_FR,
    };
    format!("{day} {} {year}", months[month - 1])
}

/// Parses `YYYY-MM-DD` into `(year, month, day)` as `usize` values.
fn parse_build_date(date: &str) -> (usize, usize, usize) {
    let mut parts = date.splitn(3, '-');
    let year = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let month = parts.next().and_then(|s| s.parse().ok()).unwrap_or(1);
    let day = parts.next().and_then(|s| s.parse().ok()).unwrap_or(1);
    (year, month, day)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_round_trips() {
        let (y, m, d) = parse_build_date("2025-01-07");
        assert_eq!((y, m, d), (2025, 1, 7));
    }

    #[test]
    fn english_format() {
        let (year, month, day) = parse_build_date("2025-08-03");
        let result = format!("{day} {} {year}", MONTHS_EN[month - 1]);
        assert_eq!(result, "3 August 2025");
    }

    #[test]
    fn french_format() {
        let (year, month, day) = parse_build_date("2025-08-03");
        let result = format!("{day} {} {year}", MONTHS_FR[month - 1]);
        assert_eq!(result, "3 août 2025");
    }
}
