#[allow(unused)]
unsafe fn foo() {
    #[allow(unsafe_code)]
    {
        #[link_section = ".init_array"]
        #[used]
        #[allow(non_upper_case_globals, non_snake_case)]
        #[doc(hidden)]
        static f: extern "C" fn() -> ::shared::__support::CtorRetType = {
            #[link_section = ".text.startup"]
            #[allow(non_snake_case)]
            extern "C" fn f() -> ::shared::__support::CtorRetType {
                unsafe {
                    do_atexit(__dtor);
                };
                ::core::default::Default::default()
            }
            f
        };
        #[link_section = ".text.exit"]
        extern "C" fn __dtor() {
            unsafe { foo() }
        }
        #[inline(always)]
        unsafe fn do_atexit(cb: unsafe extern fn()) {
            extern "C" {
                fn atexit(cb: unsafe extern fn());
            }
            unsafe {
                atexit(cb);
            }
        }
    }
    {
        {
            ::std::io::_print(format_args!("foo\n"));
        };
    }
}
