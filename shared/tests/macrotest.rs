#[cfg(target_vendor = "apple")]
#[test]
pub fn pass() {
    macrotest::expand("tests/expand-darwin/*.rs");
}
