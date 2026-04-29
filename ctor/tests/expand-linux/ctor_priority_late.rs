use ctor::ctor;

#[ctor(priority = late)]
fn foo() {
    println!("foo");
}
