use ctor::dtor;
unsafe fn foo() {
    const _: () = {
        #[link_section = ".fini_array"]
        #[used]
        #[allow(non_upper_case_globals)]
        static __DTOR__PRIVATE__REF__: extern "C" fn() = {
            #[allow(non_snake_case)]
            extern "C" fn __dtor__private__() {
                unsafe { foo() }
            }
            __dtor__private__
        };
    };
    {
        ::std::io::_print(format_args!("foo\n"));
    };
}
