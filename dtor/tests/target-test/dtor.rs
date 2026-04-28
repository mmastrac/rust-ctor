use dtor::declarative::dtor;

dtor!(
    #[dtor(unsafe)]
    fn foo() {}
);
