shared::dtor_parse!(
    #[dtor(anonymous)]
    unsafe fn foo() {
        println!("foo");
    }
);
