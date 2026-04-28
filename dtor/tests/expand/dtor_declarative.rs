use dtor::declarative::dtor;

dtor!(
    #[dtor(method = linker, link_section = ".dtors")]
    fn foo() {
        println!("foo");
    }
);
