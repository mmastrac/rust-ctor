const _: () = {
    #[allow(unused)]
    unsafe fn foo() {
        #[allow(unsafe_code)]
        {
            #[link_section = ".init_array"]
            #[used]
            #[allow(non_upper_case_globals, non_snake_case)]
            #[doc(hidden)]
            static __CTOR_FUNCTION: extern "C" fn() -> ::dtor::__support::CtorRetType = {
                #[link_section = ".text.startup"]
                #[allow(non_snake_case)]
                extern "C" fn __CTOR_FUNCTION_INNER() -> ::dtor::__support::CtorRetType {
                    unsafe {
                        ::dtor::__support::at_binary_exit(__dtor);
                    };
                    ::core::default::Default::default()
                }
                __CTOR_FUNCTION_INNER
            };
            #[link_section = ".text.exit"]
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
};
