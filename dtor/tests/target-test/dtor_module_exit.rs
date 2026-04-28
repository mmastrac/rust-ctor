use dtor::declarative::dtor;

dtor!(
    #[dtor(unsafe, method = at_module_exit)]
    fn foo_exit() {}
);
