use dtor::dtor;
fn foo() {
    const _: () = {
        #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
        #[used]
        static __CTOR__PRIVATE__REF__: unsafe extern "C" fn() = {
            unsafe extern "C" fn __ctor__private__() {
                ::dtor::__support::at_library_exit(__dtor__private__);
            }
            extern "C" fn __dtor__private__() {
                foo()
            }
            __ctor__private__
        };
    };
    {
        ::std::io::_print(format_args!("foo\n"));
    };
}
