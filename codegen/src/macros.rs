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
            $($macro_path)::+::ctor_entry!(meta=[$($meta,)*], macro_path=$($macro_path)::+, features=[$($fname,)*], imeta=$(#[$imeta])*, vis=[pub($($extra)*)], item=$($item)*);
        };
        (#[ctor $( ($meta:meta) )?] $( #[feature($fname:ident)] )* #[macro_path=$($macro_path:ident)::+] $(#[$imeta:meta])* pub $($item:tt)*) => {
            $($macro_path)::+::ctor_entry!(meta=[$($meta,)*], macro_path=$($macro_path)::+, features=[$($fname,)*], imeta=$(#[$imeta])*, vis=[pub], item=$($item)*);
        };
        (#[ctor $( ($meta:meta) )?] $( #[feature($fname:ident)] )* #[macro_path=$($macro_path:ident)::+] $(#[$imeta:meta])* fn $($item:tt)*) => {
            $($macro_path)::+::ctor_entry!(meta=[$($meta,)*], macro_path=$($macro_path)::+, features=[$($fname,)*], imeta=$(#[$imeta])*, vis=[], item=fn $($item)*);
        };
        (#[ctor $( ($meta:meta) )?] $( #[feature($fname:ident)] )* #[macro_path=$($macro_path:ident)::+] $(#[$imeta:meta])* unsafe $($item:tt)*) => {
            $($macro_path)::+::ctor_entry!(meta=[$($meta,)*], macro_path=$($macro_path)::+, features=[$($fname,)*], imeta=$(#[$imeta])*, vis=[], item=unsafe $($item)*);
        };
        (#[ctor $( ($meta:meta) )?] $( #[feature($fname:ident)] )* #[macro_path=$($macro_path:ident)::+] $(#[$imeta:meta])* static $($item:tt)*) => {
            $($macro_path)::+::ctor_entry!(meta=[$($meta,)*], macro_path=$($macro_path)::+, features=[$($fname,)*], imeta=$(#[$imeta])*, vis=[], item=static $($item)*);
        };

        (#[dtor $( ($meta:meta) )?] $( #[feature($fname:ident)] )* #[macro_path=$($macro_path:ident)::+] $(#[$imeta:meta])* pub ( $($extra:tt)* ) $($item:tt)*) => {
            $($macro_path)::+::dtor_entry!(meta=[$($meta,)*], macro_path=$($macro_path)::+, features=[$($fname,)*], imeta=$(#[$imeta])*, vis=[pub($($extra)*)], item=$($item)*);
        };
        (#[dtor $( ($meta:meta) )?] $( #[feature($fname:ident)] )* #[macro_path=$($macro_path:ident)::+] $(#[$imeta:meta])* pub $($item:tt)*) => {
            $($macro_path)::+::dtor_entry!(meta=[$($meta,)*], macro_path=$($macro_path)::+, features=[$($fname,)*], imeta=$(#[$imeta])*, vis=[pub], item=$($item)*);
        };
        (#[dtor $( ($meta:meta) )?] $( #[feature($fname:ident)] )* #[macro_path=$($macro_path:ident)::+] $(#[$imeta:meta])* fn $($item:tt)*) => {
            $($macro_path)::+::dtor_entry!(meta=[$($meta,)*], macro_path=$($macro_path)::+, features=[$($fname,)*], imeta=$(#[$imeta])*, vis=[], item=fn $($item)*);
        };
        (#[dtor $( ($meta:meta) )?] $( #[feature($fname:ident)] )* #[macro_path=$($macro_path:ident)::+] $(#[$imeta:meta])* unsafe $($item:tt)*) => {
            $($macro_path)::+::dtor_entry!(meta=[$($meta,)*], macro_path=$($macro_path)::+, features=[$($fname,)*], imeta=$(#[$imeta])*, vis=[], item=unsafe $($item)*);
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

    macro_rules! ctor_entry {
        (meta=[$($meta:meta)?], macro_path=$($macro_path:ident)::+, features=$features:tt, imeta=$(#[$fnmeta:meta])*, vis=[$($vis:tt)*], item=fn $ident:ident() $block:block) => {
            $(#[$fnmeta])*
            #[allow(unused)]
            $($vis)* fn $ident() {
                mod __ctor_internal {
                    super::$($macro_path)::+::if_has_feature!(macro_path=super::$($macro_path)::+, __warn_on_missing_unsafe, $features, {
                        #[deprecated="ctor deprecation note:\n\n \
                        Use of #[ctor] without `unsafe fn` is deprecated. As code execution before main\n\
                        is unsupported by most Rust runtime functions, these functions must be marked\n\
                        `unsafe`."]
                        const fn ctor_without_unsafe_is_deprecated() {}
                        #[allow(unused)]
                        static UNSAFE_WARNING: () = ctor_without_unsafe_is_deprecated();
                    }, {});

                    super::$($macro_path)::+::ctor_link_section!(
                        macro_path=super::$($macro_path)::+,
                        features=$features,

                        #[allow(non_upper_case_globals, non_snake_case)]
                        #[doc(hidden)]
                        static $ident: unsafe extern "C" fn() -> usize =
                        {
                            // This function must be callable from the C runtime, so it must be marked extern "C"
                            #[allow(non_snake_case)]
                            #[cfg_attr(any(target_os = "linux", target_os = "android"), link_section = ".text.startup")]
                            unsafe extern "C" fn $ident() -> usize { super::$ident(); 0 }

                            $ident
                        };
                    );
                }

                $block
            }
        };
        (meta=[$($meta:meta)?], macro_path=$($macro_path:ident)::+, features=$features:tt, imeta=$(#[$fnmeta:meta])*, vis=[$($vis:tt)*], item=unsafe fn $ident:ident() $block:block) => {
            $(#[$fnmeta])*
            #[allow(unused)]
            $($vis)* unsafe fn $ident() {
                #[doc(hidden)]
                mod __ctor_internal {
                    super::$($macro_path)::+::ctor_link_section!(
                        macro_path=super::$($macro_path)::+,
                        features=$features,

                        #[allow(non_upper_case_globals, non_snake_case)]
                        #[doc(hidden)]
                        static $ident: unsafe extern "C" fn() -> usize =
                        {
                            // This function must be callable from the C runtime, so it must be marked extern "C"
                            #[allow(non_snake_case)]
                            #[cfg_attr(any(target_os = "linux", target_os = "android"), link_section = ".text.startup")]
                            unsafe extern "C" fn $ident() -> usize { super::$ident(); 0 }

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
                    unsafe {
                        let ptr = self._storage.get();
                        let val = (&*ptr).as_ref().unwrap();
                        val
                    }
                }
            }

            impl $ident::Static<$ty> {
                fn init_once(&self) {
                    let val = Some($expr);
                    // Only write the value to `storage_ident` on startup
                    unsafe {
                        let ptr = self._storage.get();
                        ::std::ptr::write(ptr, val);
                    }
                }
            }

            #[doc(hidden)]
            #[allow(non_upper_case_globals, non_snake_case)]
            mod $ident {
                #[derive(Default)]
                #[allow(non_camel_case_types)]
                pub struct Static<T> {
                    pub _storage: ::std::cell::UnsafeCell<::std::option::Option<T>>
                }

                unsafe impl <T> std::marker::Sync for Static<T> {}

                super::$($macro_path)::+::ctor_link_section!(
                    macro_path=super::$($macro_path)::+,
                    features=$features,

                    #[allow(non_upper_case_globals, non_snake_case)]
                    #[doc(hidden)]
                    static $ident: unsafe extern "C" fn() -> usize =
                    {
                        #[allow(non_snake_case)]
                        #[cfg_attr(any(target_os = "linux", target_os = "android"), link_section = ".text.startup")]
                        unsafe extern "C" fn $ident() -> usize { super::$ident.init_once(); 0 }

                        $ident
                    };
                );
            }
        };
    }

    macro_rules! dtor_entry {
        (meta=[$($meta:meta)?], macro_path=$($macro_path:ident)::+, features=$features:tt, imeta=$(#[$fnmeta:meta])*, vis=[$($vis:tt)*], item=fn $ident:ident() $block:block) => {
            $(#[$fnmeta])*
            $($vis)* fn $ident() {
                mod __dtor_internal {
                    super::$($macro_path)::+::if_has_feature!(macro_path=super::$($macro_path)::+, __warn_on_missing_unsafe, $features, {
                        #[deprecated="dtor deprecation note:\n\n \
                        Use of #[dtor] without `unsafe fn` is deprecated. As code execution after main\n\
                        is unsupported by most Rust runtime functions, these functions must be marked\n\
                        `unsafe`."]
                        const fn dtor_without_unsafe_is_deprecated() {}
                        #[allow(unused)]
                        static UNSAFE_WARNING: () = dtor_without_unsafe_is_deprecated();
                    }, {});

                    super::$($macro_path)::+::ctor_link_section!(
                        macro_path=super::$($macro_path)::+,
                        features=$features,

                        #[allow(non_upper_case_globals, non_snake_case)]
                        #[doc(hidden)]
                        static $ident: unsafe extern "C" fn() -> usize =
                        {
                            #[allow(non_snake_case)]
                            #[cfg_attr(any(target_os = "linux", target_os = "android"), link_section = ".text.startup")]
                            unsafe extern "C" fn $ident() -> usize { do_atexit(__dtor); 0 }

                            $ident
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

                $block
            }
        };
        (meta=[$($meta:meta)?], macro_path=$($macro_path:ident)::+, features=$features:tt, imeta=$(#[$fnmeta:meta])*, vis=[$($vis:tt)?], item=unsafe fn $ident:ident() $block:block) => {
            $(#[$fnmeta])*
            $($vis)* unsafe fn $ident() {
                mod __dtor_internal {
                    super::$($macro_path)::+::ctor_link_section!(
                        macro_path=super::$($macro_path)::+,
                        features=$features,

                        #[allow(non_upper_case_globals, non_snake_case)]
                        #[doc(hidden)]
                        static $ident: unsafe extern "C" fn() -> usize =
                        {
                            #[allow(non_snake_case)]
                            #[cfg_attr(any(target_os = "linux", target_os = "android"), link_section = ".text.startup")]
                            unsafe extern "C" fn $ident() -> usize { do_atexit(__dtor); 0 }

                            $ident
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

                $block
            }
        };
    }

    /// Annotate a block with its appropriate link section.
    macro_rules! ctor_link_section {
        (macro_path=$($macro_path:ident)::+, features=$features:tt, $($block:tt)+) => {
            $($macro_path)::+::if_has_feature!(macro_path=$($macro_path)::+, used_linker, $features, {
                $($macro_path)::+::ctor_link_section_attr!(used(linker), $($block)+);
            }, {
                $($macro_path)::+::ctor_link_section_attr!(used, $($block)+);
            });
        }
    }

    macro_rules! ctor_link_section_attr {
        ($used:meta, $item:item) => {
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
        };
    }
);
