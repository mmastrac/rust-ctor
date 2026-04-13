use ctor::dtor;
#[dtor]
unsafe fn foo() {
    {
        ::std::io::_print(format_args!("foo\n"));
    };
}

ctor::declarative::dtor!(
    #[dtor]
    unsafe fn foo2() {
        {
            ::std::io::_print(format_args!("foo\n"));
        };
    }
);
