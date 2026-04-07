#[allow(unused)]
fn foo() {
    #[allow(unsafe_code)]
    {
        #[deprecated = "ctor deprecation note:\n\n \
                        Use of #[ctor] without `unsafe fn` is deprecated. As code execution before main\n\
                        is unsupported by most Rust runtime functions, these functions must be marked\n\
                        `unsafe`."]
        const fn ctor_without_unsafe_is_deprecated() {}
        #[allow(unused)]
        static UNSAFE_WARNING: () = ctor_without_unsafe_is_deprecated();
        #[link_section = ".ctors"]
        #[allow(unsafe_code)]
        #[used]
        #[allow(non_upper_case_globals, non_snake_case)]
        #[doc(hidden)]
        static __CTOR_FUNCTION: extern "C" fn() -> ::shared::__support::CtorRetType = {
            #[link_section = ".text.startup"]
            #[allow(non_snake_case)]
            extern "C" fn __CTOR_FUNCTION_INNER() -> ::shared::__support::CtorRetType {
                unsafe {
                    foo();
                };
                ::core::default::Default::default()
            }
            __CTOR_FUNCTION_INNER
        };
    }
    {
        {
            ::std::io::_print(format_args!("foo\n"));
        };
    }
}
