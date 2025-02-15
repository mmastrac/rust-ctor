::shared::ctor_parse!(
    #[ctor]
    unsafe fn foo() {
        println!("foo");
    }
);
