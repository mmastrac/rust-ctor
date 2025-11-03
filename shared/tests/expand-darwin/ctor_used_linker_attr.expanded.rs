#[allow(unused)]
fn foo() {
    #[allow(unsafe_code)]
    {
        #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
        #[used(linker)]
        #[allow(non_upper_case_globals, non_snake_case)]
        #[doc(hidden)]
        static f: extern "C" fn() -> ::shared::__support::CtorRetType = {
            #[allow(non_snake_case)]
            extern "C" fn f() -> ::shared::__support::CtorRetType {
                unsafe {
                    foo();
                };
                ::core::default::Default::default()
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
