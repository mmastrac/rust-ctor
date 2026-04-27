use dtor::dtor;

#[dtor(anonymous)]
unsafe fn foo() {
    println!("foo");
}
