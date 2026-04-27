shared::ctor_parse!(
    #[ctor(priority = 1)]
    fn foo() {
        println!("foo");
    }
);
