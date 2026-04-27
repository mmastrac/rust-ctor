use dtor::dtor;
unsafe fn foo() {
    const _: () = {
        #[link_section = ".CRT$XCU"]
        #[used]
        #[allow(non_upper_case_globals)]
        static __CTOR__PRIVATE__REF__: unsafe extern "C" fn() = {
            #[allow(non_snake_case)]
            unsafe extern "C" fn __ctor__private__() {
                ::dtor::__support::at_module_exit(__dtor__private__);
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
