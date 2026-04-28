use clitest_lib::clitest;

// Does not work on apple platforms
#[cfg(not(target_vendor = "apple"))]
clitest!(
    crt_static,
    r#"
set RUSTFLAGS "-C target-feature=+crt-static";
cd "ctor/crt-static";
defer {
    $ cargo clean --quiet
}
$ rustc -vV
%SET TARGET "$target"
*
! host: %{DATA:target}
*
$ cargo build --quiet --target $TARGET
*
$ cargo run --quiet --target $TARGET
! +crt-static
! main
"#
);

clitest!(
    edition_2018,
    r#"
set RUSTFLAGS "";
cd "ctor/edition-2018";
defer {
    $ cargo clean --quiet
}
$ cargo build --quiet
*
$ cargo run --quiet
! foo
! main
"#
);

clitest!(
    edition_2021,
    r#"
set RUSTFLAGS "";
cd "ctor/edition-2021";
defer {
    $ cargo clean --quiet
}
$ cargo build --quiet
*
$ cargo run --quiet
! foo
! main
"#
);

clitest!(
    edition_2024,
    r#"
set RUSTFLAGS "";
cd "ctor/edition-2024";
defer {
    $ cargo clean --quiet
}
$ cargo build --quiet
*
$ cargo run --quiet
! foo
! main
"#
);

clitest!(
    no_std,
    r#"
set RUSTFLAGS "";
cd "ctor/no-std";
defer {
    $ cargo clean --quiet
}
$ cargo build --quiet
*
"#
);

clitest!(
    priority,
    r#"
set RUSTFLAGS "";
cd "ctor/priority";
defer {
    $ cargo clean --quiet
}
$ cargo build --quiet
*
$ cargo run --quiet
! 1
! 2
! 3
! 4
! 5
! 6
! 7
! 8
! 9
! 10
! main
"#
);

clitest!(
    system_no_crt_static,
    r#"
set RUSTFLAGS "-C target-feature=-crt-static";
set CARGO_TARGET_DIR "target/system_no_crt_static";
cd "ctor/system";
defer {
    $ cargo clean --quiet
}
$ cargo build --lib --examples --quiet
*
$ cargo run --example dylib_load --quiet
! + ctor bin
! ++ main start
unordered {
    ! +++ ctor STATIC_INT
    ! +++ ctor lib (-crt-static)
}
unordered {
    ! -- main end
    ! --- dtor lib
}
! - dtor bin
"#
);

// Only Windows supports +crt-static w/dylibs, but we may be able to work around
// this: https://github.com/rust-lang/rust/issues/71651#issuecomment-864265118
#[cfg(windows)]
clitest!(
    system_crt_static,
    r#"
set RUSTFLAGS "-C target-feature=+crt-static";
set CARGO_TARGET_DIR "target/system_crt_static";
cd "ctor/system";
defer {
    $ cargo clean --quiet
}
$ rustc -vV
%SET TARGET "$target"
*
! host: %{DATA:target}
*
$ cargo build --lib --examples --quiet
*
$ find $CARGO_TARGET_DIR
*
$ cargo run --example dylib_load --quiet
! + ctor bin
! ++ main start
unordered {
    ! +++ ctor STATIC_INT
    ! +++ ctor lib (+crt-static)
}
unordered {
    ! -- main end
    ! --- dtor lib
}
! - dtor bin
"#
);

clitest!(
    warn_unsafe,
    r#"
set RUSTFLAGS "";
set CARGO_TARGET_DIR "target/warn_yes";
cd "ctor/warn-unsafe";
defer {
    $ cargo clean --quiet
}
$ cargo build
ignore {
    !    Compiling %{DATA}
    !     Blocking waiting for file lock on package cache
    !     Blocking waiting for file lock on shared package cache
    !      Locking %{DATA} to latest compatible version%{DATA}
}
! warning: use of deprecated function `_::ctor_without_unsafe_is_deprecated`: ctor deprecation note:
!          
!          Use of #[ctor] without `#[ctor(unsafe)]` or `unsafe fn` is deprecated. As code execution
!          before main is unsupported by most Rust runtime functions, these functions must be marked
!          `unsafe`.
if TARGET_OS == "windows" {
!  --> src\main.rs:4:1
}
if TARGET_OS != "windows" {
!  --> src/main.rs:4:1
}
!   |
! 4 | #[ctor]
!   | ^^^^^^^
!   |
!   = note: `#[warn(deprecated)]` on by default
!   = note: this warning originates in the macro `$crate::__ctor_parse_impl` which comes from the expansion of the attribute macro `ctor` (in Nightly builds, run with -Z macro-backtrace for more info)
!
! warning: use of deprecated function `_::ctor_without_unsafe_is_deprecated`: ctor deprecation note:
!          
!          Use of #[ctor] without `#[ctor(unsafe)]` or `unsafe fn` is deprecated. As code execution
!          before main is unsupported by most Rust runtime functions, these functions must be marked
!          `unsafe`.
if TARGET_OS == "windows" {
!   --> src\main.rs:21:1
}
if TARGET_OS != "windows" {
!   --> src/main.rs:21:1
}
!    |
! 21 | #[ctor]
!    | ^^^^^^^
!    |
!    = note: this warning originates in the macro `$crate::__ctor_parse_impl` which comes from the expansion of the attribute macro `ctor` (in Nightly builds, run with -Z macro-backtrace for more info)
! 
! warning: `warn-unsafe` (bin "warn-unsafe") generated 2 warnings
!     Finished `dev` profile %{DATA}
"#
);

clitest!(
    warn_unsafe_disabled,
    r#"
set RUSTFLAGS "";
set CARGO_TARGET_DIR "target/warn_no";
cd "ctor/warn-unsafe";
defer {
    $ cargo clean --quiet
}
$ cargo build --features no_warn_on_missing_unsafe
reject {
    ! warning: %{DATA}
}
*
!     Finished `dev` profile %{DATA}
"#
);

clitest!(
    no_default_features,
    r#"
set RUSTFLAGS "";
cd "ctor/no-default-features";
defer {
    $ cargo clean --quiet
}
$ cargo run --quiet
! ctor-no-default-features:ctor
! ctor-no-default-features:main
"#
);
