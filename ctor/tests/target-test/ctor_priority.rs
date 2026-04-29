use ctor::declarative::ctor;

ctor!(
    #[ctor(unsafe, priority = early)]
    fn foo() {}
);

ctor!(
    #[ctor(unsafe, priority = 1)]
    fn foo() {}
);

ctor!(
    #[ctor(unsafe, priority = late)]
    fn foo() {}
);
