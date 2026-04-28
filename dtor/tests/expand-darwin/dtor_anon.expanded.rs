use dtor::dtor;
const _: () = {
    unsafe fn foo() {
        const _: () = {
            #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
            #[used]
            static __CTOR_PRIVATE_REF: unsafe extern "C" fn() = {
                unsafe extern "C" fn __ctor_private() {
                    ::dtor::__support::at_module_exit(__dtor_private);
                }
                extern "C" fn __dtor_private() {
                    unsafe { foo() }
                }
                __ctor_private
            };
        };
        {
            ::std::io::_print(format_args!("foo\n"));
        };
    }
};
