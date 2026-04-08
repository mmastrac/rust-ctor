#[allow(unused)]
unsafe fn foo() {
    {
        {
            ::std::io::_print(format_args!("foo\n"));
        };
    }
}
const _: () = {
    #[unsafe(no_mangle)]
    #[export_name = concat!(
        "__sterm80000000_",
        module_path!(),
        "_",
        stringify!(foo),
        "_L",
        line!(),
        "C",
        column!()
    )]
    extern "C" fn aix_dtor_wrapper() {
        #[allow(unsafe_code)]
        {
            unsafe {
                foo();
            }
        }
    }
};
