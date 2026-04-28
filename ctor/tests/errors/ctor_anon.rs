use ctor::ctor;

#[ctor(anonymous)]
unsafe fn foo() {
    println!("Hello, world!");
}

#[ctor]
unsafe fn bar() {
    println!("Hello, world!");
}

fn main() {
    // Disallowed
    foo();
    // Allowed
    bar();
}
