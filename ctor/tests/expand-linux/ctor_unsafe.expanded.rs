use ctor::ctor;
unsafe fn foo() {
    {
        ::std::io::_print(format_args!("foo\n"));
    };
}
