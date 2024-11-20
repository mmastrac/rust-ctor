crate::declare_macro!(
    macro_rules! ctor_raw {
        ($used:meta $($block:tt)+) => {
            #[cfg(not(any(
                target_os = "linux",
                target_os = "android",
                target_os = "freebsd",
                target_os = "netbsd",
                target_os = "openbsd",
                target_os = "dragonfly",
                target_os = "illumos",
                target_os = "haiku",
                target_vendor = "apple",
                windows)))]
            compile_error!("#[ctor] is not supported on the current target");

            #[$used]
            #[cfg_attr(any(target_os = "linux", target_os = "android"), link_section = ".init_array")]
            #[cfg_attr(target_os = "freebsd", link_section = ".init_array")]
            #[cfg_attr(target_os = "netbsd", link_section = ".init_array")]
            #[cfg_attr(target_os = "openbsd", link_section = ".init_array")]
            #[cfg_attr(target_os = "dragonfly", link_section = ".init_array")]
            #[cfg_attr(target_os = "illumos", link_section = ".init_array")]
            #[cfg_attr(target_os = "haiku", link_section = ".init_array")]
            #[cfg_attr(target_vendor = "apple", link_section = "__DATA,__mod_init_func")]
            #[cfg_attr(windows, link_section = ".CRT$XCU")]
            $($block)+
        }
    }
);

crate::declare_macro!(
    macro_rules! ctor_impl {
        (
            fn
            macros=$macros:ident
            name=$name:ident
            used=$used:meta
            $(extra={ $($extra:item)+ })?
            item={
                $(
                    #[$meta:meta]
                )*
                $vis:vis $(unsafe)? fn $ident:ident () $(-> $ret:ty)? $block:block
            }
        ) => {
            $(#[$meta])*
            #[allow(unused)]
            $vis unsafe extern "C" fn $ident() $block

            #[doc(hidden)]
            #[allow(unused, non_snake_case)]
            mod $name {
                super::$macros::ctor_raw!($used
                #[allow(non_upper_case_globals, non_snake_case)]
                #[doc(hidden)]
                static $name: unsafe extern "C" fn() -> usize =
                {
                    #[allow(non_snake_case)]
                    #[cfg_attr(any(target_os = "linux", target_os = "android"), link_section = ".text.startup")]
                    unsafe extern "C" fn $name() -> usize { super::$ident(); 0 }

                    $name
                };
                );

                $( $($extra)+ )?
            }
        };

        (
            static
            macros=$macros:ident
            name=$name:ident
            used=$used:meta
            item={
                $(
                    #[$meta:meta]
                )*
                $vis:vis static $ident:ident: $ty:ty = $expr:expr;
            }
        ) => {

            $(#[$meta])*
            $vis static $ident: $name::Static<$ty> = $name::Static::<$ty> {
                _storage: ::std::cell::UnsafeCell::new(None)
            };

            impl ::core::ops::Deref for $name::Static<$ty> {
                type Target = $ty;
                fn deref(&self) -> &'static $ty {
                    unsafe {
                        let ptr = self._storage.get();
                        let val = (&*ptr).as_ref().unwrap();
                        val
                    }
                }
            }

            $macros::ctor_impl!(
                fn
                macros=$macros
                name=$name
                used=$used
                extra={
                    #[doc(hidden)]
                    #[derive(Default)]
                    #[allow(non_camel_case_types)]
                    pub struct Static<T> {
                        pub _storage: ::std::cell::UnsafeCell<::std::option::Option<T>>
                    }

                    unsafe impl <T> std::marker::Sync for Static<T> {}
                }
                item={
                    #[allow(non_snake_case)]
                    fn $name() {
                        let val = Some($expr);
                        // Only write the value to `storage_ident` on startup
                        unsafe {
                            let ptr = $ident._storage.get();
                            ::std::ptr::write(ptr, val);
                        }
                    }
                }
            );
        };

        (
            dtor
            macros=$macros:ident
            name=$name:ident
            used=$used:meta
            item={
                $(
                    #[$meta:meta]
                )*
                $vis:vis $(unsafe)? fn $ident:ident () $(-> $ret:ty)? $block:block
            }
        ) => {
            $(#[$meta])*
            #[allow(unused)]
            $vis unsafe extern "C" fn $ident() $block

            #[doc(hidden)]
            #[allow(unused, non_snake_case)]
            mod $name {
                super::$macros::ctor_raw!($used
                #[allow(non_upper_case_globals, non_snake_case)]
                #[doc(hidden)]
                static $name: unsafe extern "C" fn() -> usize =
                {
                    #[allow(non_snake_case)]
                    #[cfg_attr(any(target_os = "linux", target_os = "android"), link_section = ".text.startup")]
                    unsafe extern "C" fn $name() -> usize { do_atexit(__dtor); 0 }

                    $name
                };
                );

                #[cfg(not(target_vendor = "apple"))]
                #[cfg_attr(any(target_os = "linux", target_os = "android"), link_section = ".text.exit")]
                unsafe extern "C" fn __dtor() { super::$ident() }
                #[cfg(target_vendor = "apple")]
                unsafe extern "C" fn __dtor(_: *const u8) { super::$ident() }

                #[cfg(not(target_vendor = "apple"))]
                #[inline(always)]
                pub(super) unsafe fn do_atexit(cb: unsafe extern fn()) {
                    extern "C" {
                        fn atexit(cb: unsafe extern fn());
                    }
                    atexit(cb);
                }

                // For platforms that have __cxa_atexit, we register the dtor as scoped to dso_handle
                #[cfg(target_vendor = "apple")]
                #[inline(always)]
                pub(super) unsafe fn do_atexit(cb: unsafe extern fn(_: *const u8)) {
                    extern "C" {
                        static __dso_handle: *const u8;
                        fn __cxa_atexit(cb: unsafe extern fn(_: *const u8), arg: *const u8, dso_handle: *const u8);
                    }
                    __cxa_atexit(cb, core::ptr::null(), __dso_handle);
                }

            }
        };
    }
);
