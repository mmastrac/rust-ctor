use dtor::dtor;
const _: () = {
    #[deprecated = "dtor deprecation note:\n\n\
            Use of #[dtor] without `#[dtor(unsafe)]` or `unsafe fn` is deprecated. As code execution\n\
            before main is unsupported by most Rust runtime functions, these functions must be marked\n\
            `unsafe`."]
    const fn dtor_without_unsafe_is_deprecated() {}
    #[allow(unused)]
    static UNSAFE_WARNING: () = { dtor_without_unsafe_is_deprecated() };
};
#[allow(dead_code)]
fn foo() {
    fn __dtor_private_inner() {
        {
            ::std::io::_print(format_args!("foo\n"));
        };
    }
    const _: () = {
        #[link_section = ".dtors"]
        #[used]
        static __DTOR_PRIVATE_REF: extern "C" fn() = {
            extern "C" fn __dtor_private() {
                { __dtor_private_inner() }
            }
            __dtor_private
        };
    };
    { __dtor_private_inner() }
}
