use clitest_lib::clitest;

// Does not work on apple platforms
#[cfg(not(target_vendor = "apple"))]
clitest!(crt_static, r#"
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
"#);

clitest!(edition_2018, r#"
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
"#);

clitest!(edition_2021, r#"
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
"#);

clitest!(edition_2024, r#"
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
"#);

clitest!(no_std, r#"
set RUSTFLAGS "";
cd "ctor/no-std";
defer {
    $ cargo clean --quiet
}
$ cargo build --quiet
*
"#);

clitest!(system_no_crt_static, r#"
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
"#);

// Only Windows supports +crt-static w/dylibs
#[cfg(windows)]
clitest!(system_crt_static, r#"
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
$ cargo build --lib --examples --quiet --target $TARGET
*
$ cargo run --example dylib_load --quiet --target $TARGET
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
"#);

clitest!(warn_unsafe, r#"
set RUSTFLAGS "";
cd "ctor/warn-unsafe";
defer {
    $ cargo clean --quiet
}
$ cargo build
*
! warning: use of deprecated function `foo::ctor_without_unsafe_is_deprecated`: ctor deprecation note:
!          
!           Use of #[ctor] without `unsafe fn` is deprecated. As code execution before main
!          is unsupported by most Rust runtime functions, these functions must be marked
!          `unsafe`.
!  --> src/main.rs:4:1
!   |
! 4 | #[ctor]
!   | ^^^^^^^
!   |
!   = note: `#[warn(deprecated)]` on by default
!   = note: this warning originates in the macro `$crate::__support::ctor_entry` which comes from the expansion of the attribute macro `ctor` (in Nightly builds, run with -Z macro-backtrace for more info)
! 
! warning: `warn-unsafe` (bin "warn-unsafe") generated 1 warning
!     Finished `dev` profile %{DATA}
"#);
