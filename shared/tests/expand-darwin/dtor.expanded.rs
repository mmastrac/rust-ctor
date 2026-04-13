#[allow(unused)]
unsafe fn foo() {
    #[allow(unsafe_code)]
    {
        #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
        #[used]
        #[allow(non_upper_case_globals, non_snake_case)]
        #[doc(hidden)]
        static __CTOR_FUNCTION: extern "C" fn() -> ::shared::__support::CtorRetType = {
            #[allow(non_snake_case)]
            extern "C" fn __CTOR_FUNCTION_INNER() -> ::shared::__support::CtorRetType {
                unsafe {
                    ::shared::__support::at_library_exit(__dtor);
                };
                ::core::default::Default::default()
            }
            __CTOR_FUNCTION_INNER
        };
        extern "C" fn __dtor() {
            unsafe { foo() }
        }
    }
    {
        {
            ::std::io::_print(format_args!("foo\n"));
        };
    }
}
