use ctor::declarative::ctor;

ctor!(
    #[ctor(unsafe)]
    fn foo() {}
);
