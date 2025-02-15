#[cfg(not(target_family = "wasm"))]
#[allow(unused)]
fn foo() {
    #[doc(hidden)]
    #[allow(unsafe_code)]
    mod __ctor_internal {
        #[deprecated = "ctor deprecation note:\n\n \
                        Use of #[ctor] without `unsafe fn` is deprecated. As code execution before main\n\
                        is unsupported by most Rust runtime functions, these functions must be marked\n\
                        `unsafe`."]
        const fn ctor_without_unsafe_is_deprecated() {}
        #[allow(unused)]
        static UNSAFE_WARNING: () = ctor_without_unsafe_is_deprecated();
        #[link_section = "__DATA,__mod_init_func"]
        #[used]
        #[allow(non_upper_case_globals, non_snake_case)]
        #[doc(hidden)]
        static foo: extern "C" fn() -> usize = {
            #[allow(non_snake_case)]
            extern "C" fn foo() -> usize {
                unsafe {
                    super::foo();
                    0
                }
            }
            foo
        };
    }
    {
        {
            ::std::io::_print(format_args!("foo\n"));
        };
    }
}
