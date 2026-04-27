use ctor::ctor;
fn foo() {
    const _: () = {
        fn __ctor__private() {
            unsafe { foo() }
        }
        #[link_section = "__DATA,CTOR,regular,no_dead_strip"]
        #[used]
        static __CTOR__ENTRY: <::ctor::__support::explicit_ctor::CTOR as ::link_section::__support::SectionItemType>::Item = (
            __ctor__private,
            1,
        );
    };
    {
        ::std::io::_print(format_args!("foo\n"));
    };
}
