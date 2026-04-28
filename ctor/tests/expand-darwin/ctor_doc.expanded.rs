use ctor::ctor;
/// Doc 1
/// Doc 2
#[allow(dead_code)]
unsafe fn foo() {
    unsafe fn __ctor_private_inner() {
        {
            ::std::io::_print(format_args!("foo\n"));
        };
    }
    const _: () = {
        #[allow(unsafe_code)]
        #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
        #[used]
        static __CTOR_PRIVATE_REF: unsafe extern "C" fn() = {
            #[allow(unused_unsafe)]
            extern "C" fn __ctor_private() {
                { unsafe { __ctor_private_inner() } }
            }
            __ctor_private
        };
    };
    unsafe { __ctor_private_inner() }
}
