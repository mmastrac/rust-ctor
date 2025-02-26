shared::ctor_parse!(
    /// Doc 1
    #[ctor]
    /// Doc 2
    unsafe fn foo() {
        println!("foo");
    }
);
