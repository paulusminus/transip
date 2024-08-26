fn main() {
    for (key, value) in std::env::vars() {
        if key.starts_with("CARGO_CFG_TARGET_") {
            println!("{key}: {value}");
        }
    }
    // panic!("Just print the build environment");
}
