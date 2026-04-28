use dtor::dtor;
unsafe fn foo() {
    unsafe fn __dtor_private_inner() {
        {
            ::std::io::_print(format_args!("foo\n"));
        };
    }
    const _: () = {
        #[link_section = ".CRT$XCU"]
        #[used]
        static __CTOR_PRIVATE_REF: unsafe extern "C" fn() = {
            unsafe extern "C" fn __ctor_private() {
                ::dtor::__support::at_module_exit(__dtor_private);
            }
            extern "C" fn __dtor_private() {
                unsafe { __dtor_private_inner() }
            }
            __ctor_private
        };
    };
    unsafe { __dtor_private_inner() }
}
