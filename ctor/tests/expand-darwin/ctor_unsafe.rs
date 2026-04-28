use ctor::ctor;

#[ctor(unsafe)]
unsafe fn foo() {
    println!("foo");
}
