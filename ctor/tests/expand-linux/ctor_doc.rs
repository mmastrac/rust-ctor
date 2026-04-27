use ctor::ctor;

/// Doc 1
#[ctor]
/// Doc 2
unsafe fn foo() {
    println!("foo");
}
