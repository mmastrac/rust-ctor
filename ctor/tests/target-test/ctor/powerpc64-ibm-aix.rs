use ctor::declarative::ctor;

fn foo() {
    const _: () =
        {
            #[allow(unsafe_code)]
            const _: () =
                {
                    #[allow(unused_unsafe)]
                    #[no_mangle]
                    #[link_name = "__sinit0_expand_probe_expand_probe_foo_L5C1"]
                    extern "C" fn __ctor_private() { { { foo() } } }
                    __ctor_private
                };
        };
}
