use ctor::ctor;

#[ctor]
fn foo() -> u32 {
    1
}

#[ctor]
fn foo(_x: u32) {
}

struct Foo;

impl Foo {
    #[ctor]
    fn foo(self) {
    }
}

struct FooGeneric<T> {
    _t: ::std::marker::PhantomData<T>,
}

impl<T: Default> FooGeneric<T> {
    #[ctor]
    fn foo() {
        // can't use generic parameters from outer item
        _ = T::default();
    }
}

fn main() {
}
