use dtor::dtor;
/// Doc 1
/// Doc 2
#[allow(dead_code)]
unsafe fn foo() {
    unsafe fn __dtor_private_inner() {
        {
            ::std::io::_print(format_args!("foo\n"));
        };
    }
    const _: () = {
        #[link_section = ".fini_array"]
        #[used]
        static __DTOR_PRIVATE_REF: extern "C" fn() = {
            extern "C" fn __dtor_private() {
                unsafe { __dtor_private_inner() }
            }
            __dtor_private
        };
    };
    unsafe { __dtor_private_inner() }
}
