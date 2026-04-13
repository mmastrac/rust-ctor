#[allow(unused)]
fn foo() {
    #[allow(unsafe_code)]
    {
        #[deprecated = "ctor deprecation note:\n\n\
                        Use of #[ctor] without `#[ctor(unsafe)]` or `unsafe fn` is deprecated. As code execution\n\
                        before main is unsupported by most Rust runtime functions, these functions must be marked\n\
                        `unsafe`."]
        const fn ctor_without_unsafe_is_deprecated() {}
        #[allow(unused)]
        static UNSAFE_WARNING: () = ctor_without_unsafe_is_deprecated();
        (/*ERROR*/);
    }
    {
        {
            ::std::io::_print(format_args!("foo\n"));
        };
    }
}
