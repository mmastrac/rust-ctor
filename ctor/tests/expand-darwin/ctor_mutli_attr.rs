use ctor::ctor;

#[ctor(used(linker), link_section = ".ctors")]
unsafe fn foo() {
    println!("foo");
}
