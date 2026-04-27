use dtor::dtor;

#[dtor(method = link_section, link_section = ".dtors")]
fn foo() {
    println!("foo");
}
