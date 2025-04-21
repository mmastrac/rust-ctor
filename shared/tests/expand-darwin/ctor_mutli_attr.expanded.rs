#[allow(unused)]
unsafe fn foo() {
    #[allow(unsafe_code)]
    {
        #[link_section = ".ctors"]
        #[allow(unsafe_code)]
        #[used(linker)]
        #[allow(non_upper_case_globals, non_snake_case)]
        #[doc(hidden)]
        static f: extern "C" fn() -> usize = {
            #[allow(non_snake_case)]
            extern "C" fn f() -> usize {
                unsafe {
                    foo();
                    0
                }
            }
            f
        };
    }
    {
        {
            ::std::io::_print(format_args!("foo\n"));
        };
    }
}
