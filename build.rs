// The translation catalogs are read by the tr! macro at build time.
fn main() {
    println!("cargo:rerun-if-changed=i18n");
}
