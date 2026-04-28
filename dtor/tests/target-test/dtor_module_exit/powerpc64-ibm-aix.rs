use dtor::declarative::dtor;

fn foo_exit() {
    const _: () =
        {
            #[no_mangle]
            #[export_name =
            "__sinit80000000_expand_probe_expand_probe_foo_exit_L5C1"]
            unsafe extern "C" fn __ctor_private() {
                ::dtor::__support::at_module_exit(__dtor_private);
            }
            extern "C" fn __dtor_private() { { foo_exit() } }
        };
}
