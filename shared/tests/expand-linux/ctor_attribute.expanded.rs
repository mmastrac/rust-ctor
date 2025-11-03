#[allow(unused)]
fn foo() {
    #[allow(unsafe_code)]
    {
        #[link_section = ".ctors"]
        #[allow(unsafe_code)]
        #[used]
        #[allow(non_upper_case_globals, non_snake_case)]
        #[doc(hidden)]
        static f: extern "C" fn() -> ::shared::__support::CtorRetType = {
            #[link_section = ".text.startup"]
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
