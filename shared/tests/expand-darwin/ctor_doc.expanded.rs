/// Doc 1
/// Doc 2
#[allow(unused)]
unsafe fn foo() {
    #[allow(unsafe_code)]
    {
        #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
        #[used]
        #[allow(non_upper_case_globals, non_snake_case)]
        #[doc(hidden)]
        static __CTOR_FUNCTION: extern "C" fn() -> ::ctor::__support::CtorRetType = {
            #[allow(non_snake_case)]
            extern "C" fn __CTOR_FUNCTION_INNER() -> ::ctor::__support::CtorRetType {
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
