use dtor::dtor;

#[dtor(unsafe, method = link_section, link_section = ".dtors", ctor(link_section = ".ctors"), used(linker))]
fn foo() {
    println!("foo");
}
