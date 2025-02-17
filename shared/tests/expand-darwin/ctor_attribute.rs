shared::ctor_parse!(
    #[ctor(link_section = ".ctors")]
    fn foo() {
        println!("foo");
    }
);
