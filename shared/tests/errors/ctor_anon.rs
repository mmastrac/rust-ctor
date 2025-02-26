
shared::ctor_parse!(
    #[ctor(anonymous)]
    unsafe fn foo() {
        println!("Hello, world!");
    }
);

shared::ctor_parse!(
    #[ctor]
    unsafe fn bar() {
        println!("Hello, world!");
    }
);

fn main() {
    // Disallowed
    foo();
    // Allowed
    bar();
}
