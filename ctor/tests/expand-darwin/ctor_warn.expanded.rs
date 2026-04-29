use ctor::ctor;
#[allow(dead_code)]
fn foo() {
    fn __ctor_private_inner() {
        {
            ::std::io::_print(format_args!("foo\n"));
        };
    }
    const _: () = {
        #[allow(unsafe_code, unused_unsafe)]
        extern "C" fn __ctor_private() {
            { { __ctor_private_inner() } }
        }
        #[link_section = "__DATA,_CTOR0_ISIZE_FN,regular,no_dead_strip"]
        #[used]
        pub static CTOR: ::ctor::collect::Constructor = ::ctor::collect::Constructor {
            priority: (::ctor::collect::EARLY),
            ctor: __ctor_private,
        };
    };
    { __ctor_private_inner() }
}
