fn main() {
    let now = chrono::Utc::now();

    let en = now.format("%-d %B %Y").to_string();
    println!("cargo:rustc-env=BUILD_DATE_EN={en}");

    let fr = format_french_date(&now);
    println!("cargo:rustc-env=BUILD_DATE_FR={fr}");
}

fn format_french_date(dt: &chrono::DateTime<chrono::Utc>) -> String {
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

    use chrono::Datelike;
    format!(
        "{} {} {}",
        dt.day(),
        MONTHS_FR[dt.month0() as usize],
        dt.year()
    )
}
