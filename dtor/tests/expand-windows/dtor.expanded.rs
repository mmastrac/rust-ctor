use dtor::dtor;
unsafe fn foo() {
    const _: () = {
        #[link_section = ".CRT$XCU"]
        #[used]
        static __CTOR_PRIVATE_REF: unsafe extern "C" fn() = {
            unsafe extern "C" fn __ctor__private() {
                ::dtor::__support::at_module_exit(__dtor__private);
            }
            extern "C" fn __dtor__private() {
                unsafe { foo() }
            }
            __ctor__private
        };
    };
    {
        ::std::io::_print(format_args!("foo\n"));
    };
}
