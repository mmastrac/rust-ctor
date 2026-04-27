use dtor::dtor;
/// Doc 1
/// Doc 2
unsafe fn foo() {
    const _: () = {
        #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
        #[used]
        #[allow(non_upper_case_globals)]
        static __CTOR__PRIVATE__REF__: unsafe extern "C" fn() = {
            #[allow(non_snake_case)]
            unsafe extern "C" fn __ctor__private__() {
                ::dtor::__support::at_library_exit(__dtor__private__);
            }
            #[allow(non_snake_case)]
            extern "C" fn __dtor__private__() {
                unsafe { foo() }
            }
            __ctor__private__
        };
    };
    {
        ::std::io::_print(format_args!("foo\n"));
    };
}
