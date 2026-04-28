use dtor::declarative::dtor;

fn foo() {
    const _: () =
        {
            #[no_mangle]
            #[export_name = "__sterm80000000_expand_probe_foo_L5C1"]
            extern "C" fn __dtor_private() { { foo() } }
        };
}
