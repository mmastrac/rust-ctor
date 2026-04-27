use ctor::ctor;

#[ctor(link_section = ".ctors")]
fn foo() {
    println!("foo");
}
