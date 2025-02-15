::shared::ctor_parse!(
    #[feature(__warn_on_missing_unsafe)]
    #[ctor]
    fn foo() {
        println!("foo");
    }
);
