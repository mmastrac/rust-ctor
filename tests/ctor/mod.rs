use clitest_lib::clitest;

clitest!(edition_2018, r#"
set RUSTFLAGS "";
cd "ctor/edition-2018";
$ cargo build --quiet
*
$ cargo run --quiet
! foo
! main
"#);

clitest!(edition_2021, r#"
set RUSTFLAGS "";
cd "ctor/edition-2021";
$ cargo build --quiet
*
$ cargo run --quiet
! foo
! main
"#);

clitest!(edition_2024, r#"
set RUSTFLAGS "";
cd "ctor/edition-2024";
$ cargo build --quiet
*
$ cargo run --quiet
! foo
! main
"#);

clitest!(no_std, r#"
cd "ctor/no-std";
$ cargo build --quiet
*
"#);

clitest!(system, r#"
set RUSTFLAGS "";
cd "ctor/system";
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

clitest!(warn_unsafe, r#"
set RUSTFLAGS "";
cd "ctor/warn-unsafe";
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
