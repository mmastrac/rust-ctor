shared::ctor_parse!(
    #[ctor(used(linker))]
    fn foo() {
        println!("foo");
    }
);
