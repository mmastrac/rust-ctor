shared::ctor_parse!(
    #[ctor(anonymous)]
    unsafe fn foo() {
        println!("foo");
    }
);

shared::ctor_parse!(
    #[ctor(anonymous)]
    unsafe fn foo() {
        println!("foo");
    }
);
