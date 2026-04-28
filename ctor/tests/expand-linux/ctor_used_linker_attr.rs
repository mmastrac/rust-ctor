use ctor::ctor;

#[ctor(used(linker))]
fn foo() {
    println!("foo");
}
