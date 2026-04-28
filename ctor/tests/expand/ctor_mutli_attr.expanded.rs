use ctor::ctor;
unsafe fn foo() {
    const _: () = {
        #[allow(unsafe_code)]
        #[link_section = ".ctors"]
        #[used(linker)]
        static __CTOR_PRIVATE_REF: unsafe extern "C" fn() = {
            #[allow(unused_unsafe)]
            extern "C" fn __ctor_private() {
                { unsafe { foo() } }
            }
            __ctor_private
        };
    };
    {
        ::std::io::_print(format_args!("foo\n"));
    };
}
