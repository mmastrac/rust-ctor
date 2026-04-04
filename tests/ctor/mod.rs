use clitest_lib::clitest;

clitest!(edition_2018, r#"
cd "ctor/edition-2018";
$ cargo build --quiet
*
$ cargo run --quiet
! foo
! main
"#);

clitest!(edition_2021, r#"
cd "ctor/edition-2021";
$ cargo build --quiet
*
$ cargo run --quiet
! foo
! main
"#);

clitest!(edition_2024, r#"
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
cd "ctor/system";
$ cargo build --lib --examples --quiet
*
$ cargo run --example dylib_load --quiet
"""
+ ctor bin
++ main start
+++ ctor STATIC_INT
+++ ctor lib (-crt-static)
-- main end
--- dtor lib
- dtor bin
"""
"#);

clitest!(warn_unsafe, r#"
cd "ctor/warn-unsafe";
$ cargo build
*
"#);
