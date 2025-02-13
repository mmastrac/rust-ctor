#![cfg_attr(feature = "used_linker", feature(used_with_arg))]

#[ctor::ctor]
pub fn foo() {
    libc_print::libc_println!("foo");
}
