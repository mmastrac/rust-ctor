#[allow(clippy::incompatible_msrv)]
static STATIC_CTOR: STATIC_CTOR::Static<HashMap<u32, &'static str>> = STATIC_CTOR::Static::<
    HashMap<u32, &'static str>,
> {
    _storage: {
        #[link_section = ".init_array"]
        #[used]
        #[allow(non_upper_case_globals, non_snake_case)]
        #[doc(hidden)]
        static __CTOR_FUNCTION: extern "C" fn() -> ::shared::__support::CtorRetType = {
            #[link_section = ".text.startup"]
            #[allow(non_snake_case)]
            extern "C" fn __CTOR_FUNCTION_INNER() -> ::shared::__support::CtorRetType {
                _ = &*STATIC_CTOR;
                ::core::default::Default::default()
            }
            __CTOR_FUNCTION_INNER
        };
        ::std::sync::OnceLock::new()
    },
};
#[allow(clippy::incompatible_msrv)]
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
        #[allow(clippy::incompatible_msrv)]
        pub _storage: ::std::sync::OnceLock<T>,
    }
}
