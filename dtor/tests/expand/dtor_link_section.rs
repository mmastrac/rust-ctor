use dtor::dtor;

#[dtor(unsafe, method = linker, link_section = ".dtors", ctor(link_section = ".ctors"))]
fn foo() {
    println!("foo");
}
