use ctor::ctor;
use std::collections::HashMap;
static STATIC_CTOR: ::ctor::statics::Static<HashMap<u32, &'static str>> = {
    fn init() -> HashMap<u32, &'static str> {
        let m = HashMap::new();
        m
    }
    unsafe { ::ctor::statics::Static::<HashMap<u32, &'static str>>::new(init) }
};
const _: () = {
    #[link_section = "__DATA,__mod_init_func,mod_init_funcs"]
    #[used]
    #[allow(non_upper_case_globals)]
    static __CTOR__PRIVATE__REF__: unsafe extern "C" fn() = {
        #[allow(non_snake_case)]
        extern "C" fn __ctor__private__() {
            unsafe {
                _ = &*STATIC_CTOR;
            }
        }
        __ctor__private__
    };
};
