//! This is where we declare the inserted ctor macros that are inlined into each proc macro
//! invocation site. These macros are converted into token streams and written to `gen.rs`.

use crate::declare_macros;

declare_macros!(
    /// Macro entry point. At this point it's either:
    ///
    /// `#[ctor] (pub) (unsafe) fn IDENT`
    /// `#[dtor] (pub) (unsafe) fn IDENT`
    /// `#[ctor] (pub) static IDENT`
    macro_rules! ctor_parse {
        (#[ctor $( ($meta:meta) )?] $( #[feature($fname:ident)] )* #[macro_path=$($macro_path:ident)::+] $(#[$imeta:meta])* pub ( $($extra:tt)* ) $($item:tt)*) => {
            $($macro_path)::+::ctor_entry!(meta=[$($meta,)?], macro_path=$($macro_path)::+, features=[$($fname,)*], imeta=$(#[$imeta])*, vis=[pub($($extra)*)], item=$($item)*);
        };
        (#[ctor $( ($meta:meta) )?] $( #[feature($fname:ident)] )* #[macro_path=$($macro_path:ident)::+] $(#[$imeta:meta])* pub $($item:tt)*) => {
            $($macro_path)::+::ctor_entry!(meta=[$($meta,)?], macro_path=$($macro_path)::+, features=[$($fname,)*], imeta=$(#[$imeta])*, vis=[pub], item=$($item)*);
        };
        (#[ctor $( ($meta:meta) )?] $( #[feature($fname:ident)] )* #[macro_path=$($macro_path:ident)::+] $(#[$imeta:meta])* fn $($item:tt)*) => {
            $($macro_path)::+::ctor_entry!(meta=[$($meta,)?], macro_path=$($macro_path)::+, features=[$($fname,)*], imeta=$(#[$imeta])*, vis=[], item=fn $($item)*);
        };
        (#[ctor $( ($meta:meta) )?] $( #[feature($fname:ident)] )* #[macro_path=$($macro_path:ident)::+] $(#[$imeta:meta])* unsafe $($item:tt)*) => {
            $($macro_path)::+::ctor_entry!(meta=[$($meta,)?], macro_path=$($macro_path)::+, features=[$($fname,)*], imeta=$(#[$imeta])*, vis=[], item=unsafe $($item)*);
        };
        (#[ctor $( ($meta:meta) )?] $( #[feature($fname:ident)] )* #[macro_path=$($macro_path:ident)::+] $(#[$imeta:meta])* static $($item:tt)*) => {
            $($macro_path)::+::ctor_entry!(meta=[$($meta,)?], macro_path=$($macro_path)::+, features=[$($fname,)*], imeta=$(#[$imeta])*, vis=[], item=static $($item)*);
        };

        (#[dtor $( ($meta:meta) )?] $( #[feature($fname:ident)] )* #[macro_path=$($macro_path:ident)::+] $(#[$imeta:meta])* pub ( $($extra:tt)* ) $($item:tt)*) => {
            $($macro_path)::+::dtor_entry!(meta=[$($meta,)?], macro_path=$($macro_path)::+, features=[$($fname,)*], imeta=$(#[$imeta])*, vis=[pub($($extra)*)], item=$($item)*);
        };
        (#[dtor $( ($meta:meta) )?] $( #[feature($fname:ident)] )* #[macro_path=$($macro_path:ident)::+] $(#[$imeta:meta])* pub $($item:tt)*) => {
            $($macro_path)::+::dtor_entry!(meta=[$($meta,)?], macro_path=$($macro_path)::+, features=[$($fname,)*], imeta=$(#[$imeta])*, vis=[pub], item=$($item)*);
        };
        (#[dtor $( ($meta:meta) )?] $( #[feature($fname:ident)] )* #[macro_path=$($macro_path:ident)::+] $(#[$imeta:meta])* fn $($item:tt)*) => {
            $($macro_path)::+::dtor_entry!(meta=[$($meta,)?], macro_path=$($macro_path)::+, features=[$($fname,)*], imeta=$(#[$imeta])*, vis=[], item=fn $($item)*);
        };
        (#[dtor $( ($meta:meta) )?] $( #[feature($fname:ident)] )* #[macro_path=$($macro_path:ident)::+] $(#[$imeta:meta])* unsafe $($item:tt)*) => {
            $($macro_path)::+::dtor_entry!(meta=[$($meta,)?], macro_path=$($macro_path)::+, features=[$($fname,)*], imeta=$(#[$imeta])*, vis=[], item=unsafe $($item)*);
        };
    }

    /// If the features array contains the requested feature, generates `if_true`, else `if_false`.
    ///
    /// This macro matches the features recursively.
    macro_rules! if_has_feature {
        (macro_path=$($macro_path:ident)::+, used_linker, [used_linker, $(rest:ident,)*], {$($if_true:tt)*}, {$($if_false:tt)*}) => { $($if_true)* };
        (macro_path=$($macro_path:ident)::+, used_linker, [$x:ident, $($rest:ident,)*], {$($if_true:tt)*}, {$($if_false:tt)*}) => { $($macro_path)::+::if_has_feature!(macro_path=$($macro_path)::+, used_linker, [$($rest,)*], {$($if_true)*}, {$($if_false)*}); };
        (macro_path=$($macro_path:ident)::+, used_linker, [], {$($if_true:tt)*}, {$($if_false:tt)*}) => { $($if_false)* };
        (macro_path=$($macro_path:ident)::+, __warn_on_missing_unsafe, [$(a:ident,)* __warn_on_missing_unsafe, $(b:ident,)*], {$($if_true:tt)*}, {$($if_false:tt)*}) => { $($if_true)* };
        (macro_path=$($macro_path:ident)::+, __warn_on_missing_unsafe, [$x:ident, $($rest:ident,)*], {$($if_true:tt)*}, {$($if_false:tt)*}) => { $($macro_path)::+::if_has_feature!(macro_path=$($macro_path)::+, __warn_on_missing_unsafe, [$($rest,)*], {$($if_true)*}, {$($if_false)*}); };
        (macro_path=$($macro_path:ident)::+, __warn_on_missing_unsafe, [], {$($if_true:tt)*}, {$($if_false:tt)*}) => { $($if_false)* };
    }

    macro_rules! if_unsafe {
        (, {$($if_unsafe:tt)*}, {$($if_safe:tt)*}) => { $($if_safe)* };
        (unsafe, {$($if_unsafe:tt)*}, {$($if_safe:tt)*}) => { $($if_unsafe)* };
    }

    macro_rules! ctor_entry {
        (meta=[$($meta:meta)?], macro_path=$($macro_path:ident)::+, features=$features:tt, imeta=$(#[$fnmeta:meta])*, vis=[$($vis:tt)*], item=fn $ident:ident() $block:block) => {
            $($macro_path)::+::ctor_entry!(meta=[$($meta,)?], macro_path=$($macro_path)::+, features=$features, imeta=$(#[$fnmeta])*, vis=[$($vis)*], unsafe=, item=fn $ident() $block);
        };
        (meta=[$($meta:meta)?], macro_path=$($macro_path:ident)::+, features=$features:tt, imeta=$(#[$fnmeta:meta])*, vis=[$($vis:tt)*], item=unsafe fn $ident:ident() $block:block) => {
            $($macro_path)::+::ctor_entry!(meta=[$($meta,)?], macro_path=$($macro_path)::+, features=$features, imeta=$(#[$fnmeta])*, vis=[$($vis)*], unsafe=unsafe, item=fn $ident() $block);
        };
        (meta=[$($meta:meta)?], macro_path=$($macro_path:ident)::+, features=$features:tt, imeta=$(#[$fnmeta:meta])*, vis=[$($vis:tt)*], unsafe=$($unsafe:ident)?, item=fn $ident:ident() $block:block) => {
            #[cfg(target_family="wasm")]
            $(#[$fnmeta])*
            #[allow(unused)]
            #[::wasm_bindgen::prelude::wasm_bindgen(start)]
            $($vis)* fn $ident() {
                $block
            }

            #[cfg(not(target_family="wasm"))]
            $(#[$fnmeta])*
            #[allow(unused)]
            #[allow(unsafe_code)]
            $($vis)* fn $ident() {
                #[doc(hidden)]
                #[allow(unsafe_code)]
                mod __ctor_internal {
                    super::$($macro_path)::+::if_unsafe!($($unsafe)?, {}, {
                        super::$($macro_path)::+::if_has_feature!(macro_path=super::$($macro_path)::+, __warn_on_missing_unsafe, $features, {
                            #[deprecated="ctor deprecation note:\n\n \
                            Use of #[ctor] without `unsafe fn` is deprecated. As code execution before main\n\
                            is unsupported by most Rust runtime functions, these functions must be marked\n\
                            `unsafe`."]
                                const fn ctor_without_unsafe_is_deprecated() {}
                                #[allow(unused)]
                                static UNSAFE_WARNING: () = ctor_without_unsafe_is_deprecated();
                        }, {});
                    });

                    super::$($macro_path)::+::ctor_link_section!(
                        array,
                        macro_path=super::$($macro_path)::+,
                        features=$features,

                        #[allow(non_upper_case_globals, non_snake_case)]
                        #[doc(hidden)]
                        static $ident: unsafe extern "C" fn() -> usize =
                        {
                            super::$($macro_path)::+::ctor_link_section!(
                                startup,
                                macro_path=super::$($macro_path)::+,
                                features=$features,

                                #[allow(non_snake_case)]
                                unsafe extern "C" fn $ident() -> usize { super::$ident(); 0 }
                            );

                            $ident
                        };
                    );
                }

                $block
            }
        };
        (meta=[$($meta:meta)?], macro_path=$($macro_path:ident)::+, features=$features:tt, imeta=$(#[$imeta:meta])*, vis=[$($vis:tt)*], item=static $ident:ident : $ty:ty = $expr:expr;) => {
            $(#[$imeta])*
            $($vis)* static $ident: $ident::Static<$ty> = $ident::Static::<$ty> {
                _storage: ::std::cell::UnsafeCell::new(None)
            };

            impl ::core::ops::Deref for $ident::Static<$ty> {
                type Target = $ty;
                fn deref(&self) -> &'static $ty {
                    #[allow(unsafe_code)]
                    unsafe {
                        let ptr = self._storage.get();
                        let val = (&*ptr).as_ref().unwrap();
                        val
                    }
                }
            }

            impl $ident::Static<$ty> {
                #[allow(dead_code)]
                fn init_once(&self) {
                    let val = Some($expr);
                    // Only write the value to `storage_ident` on startup
                    #[allow(unsafe_code)]
                    unsafe {
                        let ptr = self._storage.get();
                        ::std::ptr::write(ptr, val);
                    }
                }
            }

            #[doc(hidden)]
            #[allow(non_upper_case_globals, non_snake_case)]
            #[allow(unsafe_code)]
            mod $ident {
                #[allow(non_camel_case_types, unreachable_pub)]
                pub struct Static<T> {
                    pub _storage: ::std::cell::UnsafeCell<::std::option::Option<T>>
                }

                #[allow(unsafe_code)]
                unsafe impl <T> std::marker::Sync for Static<T> {}

                #[cfg(target_family="wasm")]
                #[::wasm_bindgen::prelude::wasm_bindgen(start)]
                fn init() {
                    super::$ident.init_once();
                }

                #[cfg(not(target_family="wasm"))]
                super::$($macro_path)::+::ctor_link_section!(
                    array,
                    macro_path=super::$($macro_path)::+,
                    features=$features,

                    #[allow(non_upper_case_globals, non_snake_case)]
                    #[doc(hidden)]
                    static $ident: unsafe extern "C" fn() -> usize =
                    {
                        super::$($macro_path)::+::ctor_link_section!(
                            startup,
                            macro_path=super::$($macro_path)::+,
                            features=$features,

                            #[allow(non_snake_case)]
                            unsafe extern "C" fn $ident() -> usize { super::$ident.init_once(); 0 }
                        );

                        $ident
                    };
                );
            }
        };
    }

    macro_rules! dtor_entry {
        (meta=[$($meta:meta)?], macro_path=$($macro_path:ident)::+, features=$features:tt, imeta=$(#[$fnmeta:meta])*, vis=[$($vis:tt)*], item=fn $ident:ident() $block:block) => {
            $($macro_path)::+::dtor_entry!(meta=[$($meta,)?], macro_path=$($macro_path)::+, features=$features, imeta=$(#[$fnmeta])*, vis=[$($vis)*], unsafe=, item=fn $ident() $block);
        };
        (meta=[$($meta:meta)?], macro_path=$($macro_path:ident)::+, features=$features:tt, imeta=$(#[$fnmeta:meta])*, vis=[$($vis:tt)*], item=unsafe fn $ident:ident() $block:block) => {
            $($macro_path)::+::dtor_entry!(meta=[$($meta,)?], macro_path=$($macro_path)::+, features=$features, imeta=$(#[$fnmeta])*, vis=[$($vis)*], unsafe=unsafe, item=fn $ident() $block);
        };
        (meta=[$($meta:meta)?], macro_path=$($macro_path:ident)::+, features=$features:tt, imeta=$(#[$fnmeta:meta])*, vis=[$($vis:tt)*], unsafe=$($unsafe:ident)?, item=fn $ident:ident() $block:block) => {
            $(#[$fnmeta])*
            #[allow(unsafe_code, unused)]
            $($vis)* fn $ident() {
                mod __dtor_internal {
                    super::$($macro_path)::+::if_unsafe!($($unsafe)?, {}, {
                        super::$($macro_path)::+::if_has_feature!(macro_path=super::$($macro_path)::+, __warn_on_missing_unsafe, $features, {
                            #[deprecated="dtor deprecation note:\n\n \
                            Use of #[dtor] without `unsafe fn` is deprecated. As code execution after main\n\
                            is unsupported by most Rust runtime functions, these functions must be marked\n\
                            `unsafe`."]
                            const fn dtor_without_unsafe_is_deprecated() {}
                            #[allow(unused)]
                            static UNSAFE_WARNING: () = dtor_without_unsafe_is_deprecated();
                        }, {});
                    });

                    super::$($macro_path)::+::ctor_link_section!(
                        array,
                        macro_path=super::$($macro_path)::+,
                        features=$features,

                        #[allow(non_upper_case_globals, non_snake_case)]
                        #[doc(hidden)]
                        static $ident: unsafe extern "C" fn() -> usize =
                        {
                            super::$($macro_path)::+::ctor_link_section!(
                                startup,
                                macro_path=super::$($macro_path)::+,
                                features=$features,

                                #[allow(non_snake_case)]
                                unsafe extern "C" fn $ident() -> usize { unsafe { do_atexit(__dtor); 0 } }
                            );

                            $ident
                        };
                    );

                    super::$($macro_path)::+::ctor_link_section!(
                        exit,
                        macro_path=super::$($macro_path)::+,
                        features=$features,

                        unsafe extern "C" fn __dtor(
                            #[cfg(target_vendor = "apple")] _: *const u8
                        ) { super::$ident() }
                    );

                    #[cfg(not(target_vendor = "apple"))]
                    #[inline(always)]
                    pub(super) unsafe fn do_atexit(cb: unsafe extern fn()) {
                        unsafe extern "C" {
                            fn atexit(cb: unsafe extern fn());
                        }
                        unsafe {
                            atexit(cb);
                        }
                    }

                    // For platforms that have __cxa_atexit, we register the dtor as scoped to dso_handle
                    #[cfg(target_vendor = "apple")]
                    #[inline(always)]
                    pub(super) unsafe fn do_atexit(cb: unsafe extern "C" fn(_: *const u8)) {
                        unsafe extern "C" {
                            static __dso_handle: *const u8;
                            fn __cxa_atexit(cb: unsafe extern "C" fn(_: *const u8), arg: *const u8, dso_handle: *const u8);
                        }
                        unsafe {
                            __cxa_atexit(cb, core::ptr::null(), __dso_handle);
                        }
                    }
                }

                $block
            }
        };
    }

    /// Annotate a block with its appropriate link section.
    macro_rules! ctor_link_section {
        ($section:ident,macro_path=$($macro_path:ident)::+, features=$features:tt, $($block:tt)+) => {
            $($macro_path)::+::if_has_feature!(macro_path=$($macro_path)::+, used_linker, $features, {
                $($macro_path)::+::ctor_link_section_attr!($section, const {1}, used(linker), $($block)+);
            }, {
                $($macro_path)::+::ctor_link_section_attr!($section, const {1}, used, $($block)+);
            });

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
                windows
            )))]
            compile_error!("#[ctor]/#[dtor] is not supported on the current target");
        }
    }

    /// Depending on the edition, we use either the top or bottom path because
    /// of the change in how `const {1}` is parsed in edition 2021/2024 macros.
    ///
    /// Because of some strangeness around clippy validation of unsafe(...)
    /// attributes, we just skip them entirely when clippy-ing.
    macro_rules! ctor_link_section_attr {
        (array, $e:expr, $used:meta, $item:item) => {
            #[allow(unsafe_code)]
            #[cfg_attr(
                any(
                    target_os = "linux",
                    target_os = "android",
                    target_os = "freebsd",
                    target_os = "netbsd",
                    target_os = "openbsd",
                    target_os = "dragonfly",
                    target_os = "illumos",
                    target_os = "haiku"
                ),
                unsafe(link_section = ".init_array")
            )]
            #[cfg_attr(target_vendor = "apple", unsafe(link_section = "__DATA,__mod_init_func"))]
            #[cfg_attr(windows, unsafe(link_section = ".CRT$XCU"))]
            #[$used]
            $item
        };
        (array, const $e:expr, $used:meta, $item:item) => {
            #[cfg(not(clippy))]
            #[allow(unsafe_code)]
            #[cfg_attr(
                any(
                    target_os = "linux",
                    target_os = "android",
                    target_os = "freebsd",
                    target_os = "netbsd",
                    target_os = "openbsd",
                    target_os = "dragonfly",
                    target_os = "illumos",
                    target_os = "haiku"
                ),
                link_section = ".init_array"
            )]
            #[cfg_attr(target_vendor = "apple", link_section = "__DATA,__mod_init_func")]
            #[cfg_attr(windows, link_section = ".CRT$XCU")]
            #[$used]
            $item

            #[cfg(clippy)]
            #[used]
            $item
        };
        (startup, $e:expr, $used:meta, $item:item) => {
            #[cfg(not(clippy))]
            #[cfg_attr(any(target_os = "linux", target_os = "android"), link_section = ".text.startup")]
            $item

            #[cfg(clippy)]
            $item
        };
        (startup, const $e:expr, $used:meta, $item:item) => {
            #[cfg_attr(any(target_os = "linux", target_os = "android"), unsafe(link_section = ".text.startup"))]
            $item
        };
        (exit, $e:expr, $used:meta, $item:item) => {
            #[cfg(not(clippy))]
            #[cfg_attr(any(target_os = "linux", target_os = "android"), link_section = ".text.exit")]
            $item

            #[cfg(clippy)]
            $item
        };
        (exit, const $e:expr, $used:meta, $item:item) => {
            #[cfg_attr(any(target_os = "linux", target_os = "android"), unsafe(link_section = ".text.exit"))]
            $item
        };
    }
);
