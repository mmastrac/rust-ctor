#[cfg(not(target_family = "wasm"))]
#[allow(unused)]
fn foo() {
    #[doc(hidden)]
    /// Internal module.
    ///features=[(link_section = ".ctors"),]
    #[allow(unsafe_code)]
    mod __ctor_internal {
        #[link_section = ".ctors"]
        #[allow(unsafe_code)]
        #[used]
        #[allow(non_upper_case_globals, non_snake_case)]
        #[doc(hidden)]
        static foo: extern "C" fn() -> usize = {
            #[allow(non_snake_case)]
            extern "C" fn foo() -> usize {
                unsafe {
                    super::foo();
                    0
                }
            }
            foo
        };
    }
    {
        {
            ::std::io::_print(format_args!("foo\n"));
        };
    }
}
