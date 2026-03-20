fn main() {
    let now = chrono::Utc::now();
    println!("cargo:rustc-env=BUILD_DATE={}", now.format("%Y-%m-%d"));
}
