use clitest_lib::clitest;

clitest!(
    no_default_features,
    r#"
set RUSTFLAGS "";
cd "dtor/no-default-features";
defer {
    $ cargo clean --quiet
}
$ cargo run --quiet
! dtor-no-default-features:main
! dtor-no-default-features:dtor
"#
);
