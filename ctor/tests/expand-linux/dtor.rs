use ctor::dtor;

#[dtor]
unsafe fn foo() {
    println!("foo");
}
