#[::dtor::dtor(unsafe, method = link_section, link_section = ".dtors")]
fn foo() {
    println!("foo");
}

#[::dtor::dtor(unsafe, method = link_section, link_section = ".dtors", crate_path = ::dtor)]
fn bar() {
    println!("foo");
}
