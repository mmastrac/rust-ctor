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

/// This should also not warn
#[ctor(unsafe)]
fn bar2() {
    println!("bar2");
}

fn main() {
}
