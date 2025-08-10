#[cfg(target_vendor = "apple")]
#[test]
pub fn pass_darwin() {
    macrotest::expand("tests/expand-darwin/*.rs");
}

#[cfg(target_os = "linux")]
#[test]
pub fn pass_linux() {
    macrotest::expand("tests/expand-linux/*.rs");
}

#[test]
pub fn pass() {
    macrotest::expand("tests/expand/*.rs");
}

#[test]
pub fn trybuild() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/errors/*.rs");
    t.pass("tests/expand/*.rs");
}
