struct Foo {
}

impl Foo {
    #[ctor(unsafe, link_section = ".ctors")]
    fn ctor() {
        libc_eprintln!("Foo::ctor");
    }
}
