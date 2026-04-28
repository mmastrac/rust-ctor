#[::dtor::dtor(unsafe, method = linker, link_section = ".dtors")]
fn foo() {
    println!("foo");
}

#[::dtor::dtor(unsafe, method = linker, link_section = ".dtors", crate_path = ::dtor)]
fn bar() {
    println!("foo");
}
