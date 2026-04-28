use ctor::ctor;
fn foo() {
    const _: () = {
        #[allow(unused_unsafe)]
        fn __ctor_private() {
            unsafe { foo() }
        }
        #[link_section = "__DATA,CTOR,regular,no_dead_strip"]
        #[used]
        static __CTOR_ENTRY: (fn(), u16) = (__ctor_private, 1);
    };
    {
        ::std::io::_print(format_args!("foo\n"));
    };
}
