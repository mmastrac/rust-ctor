use dtor::dtor;
fn foo() {
    const _: () = {
        #[link_section = ".dtors"]
        #[used]
        static __DTOR_PRIVATE_REF: extern "C" fn() = {
            extern "C" fn __dtor_private() {
                { foo() }
            }
            __dtor_private
        };
    };
    {
        ::std::io::_print(format_args!("foo\n"));
    };
}
