use dtor::declarative::dtor;

fn foo() {
    const _: () =
        {
            #[no_mangle]
            #[link_name = "__sterm80000000"]
            extern "C" fn __dtor_private() { { foo() } }
        };
}
