#[test]
#[cfg(feature = "help")]
#[cfg(feature = "error-context")]
#[cfg(feature = "usage")]
fn ui_tests() {
    let t = trycmd::TestCases::new();
    let features = [
        // Default
        #[cfg(feature = "std")]
        "std",
        #[cfg(feature = "color")]
        "color",
        #[cfg(feature = "help")]
        "help",
        #[cfg(feature = "usage")]
        "usage",
        #[cfg(feature = "error-context")]
        "error-context",
        #[cfg(feature = "suggestions")]
        "suggestions",
        // Optional
        #[cfg(feature = "derive")]
        "derive",
        #[cfg(feature = "wrap_help")]
        "wrap_help",
    ]
    .join(" ");
    t.register_bins(trycmd::cargo::compile_examples(["--features", &features]).unwrap());
    t.case("tests/ui/*.toml");
}
