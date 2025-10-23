pub fn print_version() {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");

    println!("{name:?} {version:?}");
}
