use dtor::dtor;

#[dtor(unsafe, link_section = ".dtors", ctor(link_section = ".ctors"))]
fn foo() {
    println!("foo");
}
