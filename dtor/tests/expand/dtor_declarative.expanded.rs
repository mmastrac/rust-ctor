use dtor::declarative::dtor;
const _: () = {
    #[deprecated = "dtor deprecation note:\n\n\
                Use of #[dtor] without `#[dtor(unsafe)]` or `unsafe fn` is deprecated. As code execution\n\
                before main is unsupported by most Rust runtime functions, these functions must be marked\n\
                `unsafe`."]
    const fn dtor_without_unsafe_is_deprecated() {}
    #[allow(unused)]
    static UNSAFE_WARNING: () = { dtor_without_unsafe_is_deprecated() };
};
fn foo() {
    const _: () = {
        #[link_section = ".dtors"]
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
