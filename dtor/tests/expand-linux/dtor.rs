use dtor::dtor;

#[dtor]
unsafe fn foo() {
    println!("foo");
}
