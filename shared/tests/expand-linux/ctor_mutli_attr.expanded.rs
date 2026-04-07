#[allow(unused)]
unsafe fn foo() {
    #[allow(unsafe_code)]
    {
        #[link_section = ".ctors"]
        #[allow(unsafe_code)]
        #[used(linker)]
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
