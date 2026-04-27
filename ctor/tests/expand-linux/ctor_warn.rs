use ctor::ctor;

#[ctor]
fn foo() {
    println!("foo");
}
