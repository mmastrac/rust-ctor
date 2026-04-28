use ctor::ctor;

#[ctor(anonymous)]
unsafe fn foo() {
    println!("foo");
}

#[ctor(anonymous)]
unsafe fn foo() {
    println!("foo");
}