#[allow(unused)]
fn foo() {
    #[allow(unsafe_code)]
    {
        #[deprecated = "ctor deprecation note:\n\n \
                        Use of #[ctor] without `unsafe fn` is deprecated. As code execution before main\n\
                        is unsupported by most Rust runtime functions, these functions must be marked\n\
                        `unsafe`."]
        const fn ctor_without_unsafe_is_deprecated() {}
        #[allow(unused)]
        static UNSAFE_WARNING: () = ctor_without_unsafe_is_deprecated();
        const _: () = {
            #[link_section = "__DATA,CTOR,regular,no_dead_strip"]
            #[used]
            static ANONYMOUS: <::shared::__support::CTOR as ::link_section::__support::SectionItemType>::Item = (
                {
                    fn ctor() {
                        unsafe {
                            foo();
                        }
                    }
                    ctor
                },
                1,
            );
        };
    }
    {
        {
            ::std::io::_print(format_args!("foo\n"));
        };
    }
}
