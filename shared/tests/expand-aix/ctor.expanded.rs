#[allow(unused)]
unsafe fn foo() {
    {
        {
            ::std::io::_print(format_args!("foo\n"));
        };
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
mod __aix_ctor_impl {
    use super::*;
    #[unsafe(no_mangle)]
    #[export_name = concat!("__sinit80000000_", stringify!(foo))]
    extern "C" fn aix_ctor_wrapper() {
        #[allow(unsafe_code)]
        {
            unsafe {
                foo();
            }
        }
    }
}
