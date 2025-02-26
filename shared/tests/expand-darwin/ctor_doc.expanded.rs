#[cfg(not(target_family = "wasm"))]
/// Doc 1
/// Doc 2
#[allow(unused)]
unsafe fn foo() {
    #[doc(hidden)]
    /// Internal module.
    ///features=[]
    #[allow(unsafe_code)]
    mod __ctor_internal {
        #[link_section = "__DATA,__mod_init_func"]
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
