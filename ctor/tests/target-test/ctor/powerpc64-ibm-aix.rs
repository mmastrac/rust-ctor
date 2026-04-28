use ctor::declarative::ctor;

#[allow(dead_code)]
fn foo() {
    fn __ctor_private_inner() {}
    const _: () =
        {
            #[allow(unsafe_code)]
            const _: () =
                {
                    #[allow(unused_unsafe)]
                    #[no_mangle]
                    #[export_name =
                    "__sinit0_expand_probe_expand_probe_foo_L5C1"]
                    extern "C" fn __ctor_private() {
                        { { __ctor_private_inner() } }
                    }
                    __ctor_private
                };
        };
    { __ctor_private_inner() }
}
