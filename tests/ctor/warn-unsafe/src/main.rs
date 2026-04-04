use ctor::ctor;

/// This should warn
#[ctor]
fn foo() {
    println!("foo");
}

/// This should not warn
#[ctor]
unsafe fn bar() {
    println!("bar");
}

fn main() {
}
