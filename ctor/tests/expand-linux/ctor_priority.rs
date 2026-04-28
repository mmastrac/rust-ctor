use ctor::ctor;

#[ctor(priority = 1)]
fn foo() {
    println!("foo");
}
