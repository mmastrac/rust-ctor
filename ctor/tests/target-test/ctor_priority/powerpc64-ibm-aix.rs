use ctor::declarative::ctor;




#[allow(dead_code)]
fn early() {
    fn __ctor_private_inner() {}
    const _: () =
        {
            #[allow(unsafe_code)]
            const _: () =
                {
                    #[allow(unused_unsafe)]
                    #[no_mangle]
                    #[export_name =
                    "__sinit0_expand_probe_expand_probe_early_L5C1"]
                    extern "C" fn __ctor_private() {
                        { { __ctor_private_inner() } }
                    }
                    __ctor_private
                };
        };
    { __ctor_private_inner() }
}
#[allow(dead_code)]
fn priority1() {
    fn __ctor_private_inner() {}
    const _: () =
        {
            #[allow(unsafe_code)]
            const _: () =
                {
                    #[allow(unused_unsafe)]
                    #[no_mangle]
                    #[export_name =
                    "__sinit1_expand_probe_expand_probe_priority1_L10C1"]
                    extern "C" fn __ctor_private() {
                        { { __ctor_private_inner() } }
                    }
                    __ctor_private
                };
        };
    { __ctor_private_inner() }
}
#[allow(dead_code)]
fn late() {
    fn __ctor_private_inner() {}
    const _: () =
        {
            #[allow(unsafe_code)]
            const _: () =
                {
                    #[allow(unused_unsafe)]
                    #[no_mangle]
                    #[export_name =
                    "__sinit65535_expand_probe_expand_probe_late_L15C1"]
                    extern "C" fn __ctor_private() {
                        { { __ctor_private_inner() } }
                    }
                    __ctor_private
                };
        };
    { __ctor_private_inner() }
}
#[allow(dead_code)]
fn naked() {
    fn __ctor_private_inner() {}
    const _: () =
        {
            #[allow(unsafe_code)]
            const _: () =
                {
                    #[allow(unused_unsafe)]
                    #[no_mangle]
                    #[export_name =
                    "__sinit0_expand_probe_expand_probe_naked_L20C1"]
                    extern "C" fn __ctor_private() {
                        { { __ctor_private_inner() } }
                    }
                    __ctor_private
                };
        };
    { __ctor_private_inner() }
}
