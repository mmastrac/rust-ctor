use dtor::dtor;
/// Doc 1
/// Doc 2
unsafe fn foo() {
    const _: () = {
        #[link_section = ".fini_array"]
        #[used]
        static __DTOR_PRIVATE_REF: extern "C" fn() = {
            extern "C" fn __dtor_private() {
                unsafe { foo() }
            }
            __dtor_private
        };
    };
    {
        ::std::io::_print(format_args!("foo\n"));
    };
}
