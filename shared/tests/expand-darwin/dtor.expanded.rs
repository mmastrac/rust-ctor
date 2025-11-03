#[allow(unused)]
unsafe fn foo() {
    #[allow(unsafe_code)]
    {
        #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
        #[used]
        #[allow(non_upper_case_globals, non_snake_case)]
        #[doc(hidden)]
        static f: extern "C" fn() -> ::shared::__support::CtorRetType = {
            #[allow(non_snake_case)]
            extern "C" fn f() -> ::shared::__support::CtorRetType {
                unsafe {
                    do_atexit(__dtor);
                };
                ::core::default::Default::default()
            }
            f
        };
        extern "C" fn __dtor(_: *const u8) {
            unsafe { foo() }
        }
        #[inline(always)]
        unsafe fn do_atexit(cb: extern "C" fn(_: *const u8)) {
            extern "C" {
                static __dso_handle: *const u8;
                fn __cxa_atexit(
                    cb: extern "C" fn(_: *const u8),
                    arg: *const u8,
                    dso_handle: *const u8,
                );
            }
            unsafe {
                __cxa_atexit(cb, ::core::ptr::null(), __dso_handle);
            }
        }
    }
    {
        {
            ::std::io::_print(format_args!("foo\n"));
        };
    }
}
