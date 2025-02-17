shared::ctor_parse!(
    #[ctor(used(linker), link_section = ".ctors")]
    unsafe fn foo() {
        println!("foo");
    }
);
