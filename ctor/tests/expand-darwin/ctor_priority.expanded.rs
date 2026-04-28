use ctor::ctor;
#[allow(dead_code)]
fn foo() {
    fn __ctor_private_inner() {
        {
            ::std::io::_print(format_args!("foo\n"));
        };
    }
    const _: () = {
        #[allow(unused_unsafe)]
        fn __ctor_private() {
            { { __ctor_private_inner() } }
        }
        #[link_section = "__DATA,CTOR,regular,no_dead_strip"]
        #[used]
        static __CTOR_ENTRY: (fn(), u16) = (__ctor_private, 1);
    };
    { __ctor_private_inner() }
}
