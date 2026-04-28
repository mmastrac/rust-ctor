use dtor::dtor;
#[allow(dead_code)]
fn foo() {
    fn __dtor_private_inner() {
        {
            ::std::io::_print(format_args!("foo\n"));
        };
    }
    const _: () = {
        #[link_section = ".dtors"]
        #[used(linker)]
        static __DTOR_PRIVATE_REF: extern "C" fn() = {
            extern "C" fn __dtor_private() {
                { __dtor_private_inner() }
            }
            __dtor_private
        };
    };
    { __dtor_private_inner() }
}
