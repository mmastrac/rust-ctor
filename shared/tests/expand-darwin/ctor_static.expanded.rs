static STATIC_CTOR: STATIC_CTOR::Static<HashMap<u32, &'static str>> = STATIC_CTOR::Static::<
    HashMap<u32, &'static str>,
> {
    _storage: {
        #[link_section = "__DATA,__mod_init_func"]
        #[used]
        #[allow(non_upper_case_globals, non_snake_case)]
        #[doc(hidden)]
        static f: extern "C" fn() -> ::shared::__support::CtorRetType = {
            #[allow(non_snake_case)]
            extern "C" fn f() -> ::shared::__support::CtorRetType {
                _ = &*STATIC_CTOR;
                core::default::Default::default()
            }
            f
        };
        ::std::sync::OnceLock::new()
    },
};
impl ::core::ops::Deref for STATIC_CTOR::Static<HashMap<u32, &'static str>> {
    type Target = HashMap<u32, &'static str>;
    fn deref(&self) -> &HashMap<u32, &'static str> {
        fn init() -> HashMap<u32, &'static str> {
            let m = HashMap::new();
            m
        }
        self._storage.get_or_init(move || { init() })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, non_snake_case)]
#[allow(unsafe_code)]
mod STATIC_CTOR {
    #[allow(non_camel_case_types, unreachable_pub)]
    pub struct Static<T> {
        pub _storage: ::std::sync::OnceLock<T>,
    }
}
