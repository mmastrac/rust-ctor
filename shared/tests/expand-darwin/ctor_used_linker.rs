shared::ctor_parse!(
    #[feature(used_linker)]
    #[ctor]
    fn foo() {
        println!("foo");
    }
);
