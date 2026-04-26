use dtor::dtor;

#[dtor(unsafe, link_section = ".dtors", ctor(link_section = ".ctors"), used(linker))]
fn foo() {
    println!("foo");
}
