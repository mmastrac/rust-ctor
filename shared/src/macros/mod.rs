/// Macro entry point. At this point it's either:
///
/// `#[ctor] (pub) (unsafe) fn IDENT`
/// `#[dtor] (pub) (unsafe) fn IDENT`
/// `#[ctor] (pub) static IDENT`
#[doc(hidden)]
#[macro_export]
macro_rules! __ctor_parse {
    ($( #[feature($fname:ident)] )* #[ctor $( ($meta:meta) )?] $(#[$imeta:meta])* pub ( $($extra:tt)* ) $($item:tt)*) => {
        $crate::__support::ctor_entry!(meta=[$($meta,)?], features=[$($fname,)*], imeta=$(#[$imeta])*, vis=[pub($($extra)*)], item=$($item)*);
    };
    ($( #[feature($fname:ident)] )* #[ctor $( ($meta:meta) )?] $(#[$imeta:meta])* pub $($item:tt)*) => {
        $crate::__support::ctor_entry!(meta=[$($meta,)?], features=[$($fname,)*], imeta=$(#[$imeta])*, vis=[pub], item=$($item)*);
    };
    ($( #[feature($fname:ident)] )* #[ctor $( ($meta:meta) )?] $(#[$imeta:meta])* fn $($item:tt)*) => {
        $crate::__support::ctor_entry!(meta=[$($meta,)?], features=[$($fname,)*], imeta=$(#[$imeta])*, vis=[], item=fn $($item)*);
    };
    ($( #[feature($fname:ident)] )* #[ctor $( ($meta:meta) )?] $(#[$imeta:meta])* unsafe $($item:tt)*) => {
        $crate::__support::ctor_entry!(meta=[$($meta,)?], features=[$($fname,)*], imeta=$(#[$imeta])*, vis=[], item=unsafe $($item)*);
    };
    ($( #[feature($fname:ident)] )* #[ctor $( ($meta:meta) )?] $(#[$imeta:meta])* static $($item:tt)*) => {
        $crate::__support::ctor_entry!(meta=[$($meta,)?], features=[$($fname,)*], imeta=$(#[$imeta])*, vis=[], item=static $($item)*);
    };

    ($( #[feature($fname:ident)] )* #[dtor $( ($meta:meta) )?] $(#[$imeta:meta])* pub ( $($extra:tt)* ) $($item:tt)*) => {
        $crate::__support::dtor_entry!(meta=[$($meta,)?], features=[$($fname,)*], imeta=$(#[$imeta])*, vis=[pub($($extra)*)], item=$($item)*);
    };
    ($( #[feature($fname:ident)] )* #[dtor $( ($meta:meta) )?] $(#[$imeta:meta])* pub $($item:tt)*) => {
        $crate::__support::dtor_entry!(meta=[$($meta,)?], features=[$($fname,)*], imeta=$(#[$imeta])*, vis=[pub], item=$($item)*);
    };
    ($( #[feature($fname:ident)] )* #[dtor $( ($meta:meta) )?] $(#[$imeta:meta])* fn $($item:tt)*) => {
        $crate::__support::dtor_entry!(meta=[$($meta,)?], features=[$($fname,)*], imeta=$(#[$imeta])*, vis=[], item=fn $($item)*);
    };
    ($( #[feature($fname:ident)] )* #[dtor $( ($meta:meta) )?] $(#[$imeta:meta])* unsafe $($item:tt)*) => {
        $crate::__support::dtor_entry!(meta=[$($meta,)?], features=[$($fname,)*], imeta=$(#[$imeta])*, vis=[], item=unsafe $($item)*);
    };
}

/// If the features array contains the requested feature, generates `if_true`, else `if_false`.
///
/// This macro matches the features recursively.
#[doc(hidden)]
#[macro_export]
macro_rules! __if_has_feature {
    (used_linker, [used_linker, $(rest:ident,)*], {$($if_true:tt)*}, {$($if_false:tt)*}) => { $($if_true)* };
    (used_linker, [$x:ident, $($rest:ident,)*], {$($if_true:tt)*}, {$($if_false:tt)*}) => { $crate::__support::if_has_feature!(used_linker, [$($rest,)*], {$($if_true)*}, {$($if_false)*}); };
    (used_linker, [], {$($if_true:tt)*}, {$($if_false:tt)*}) => { $($if_false)* };
    (__warn_on_missing_unsafe, [$(a:ident,)* __warn_on_missing_unsafe, $(b:ident,)*], {$($if_true:tt)*}, {$($if_false:tt)*}) => { $($if_true)* };
    (__warn_on_missing_unsafe, [$x:ident, $($rest:ident,)*], {$($if_true:tt)*}, {$($if_false:tt)*}) => { $crate::__support::if_has_feature!(__warn_on_missing_unsafe, [$($rest,)*], {$($if_true)*}, {$($if_false)*}); };
    (__warn_on_missing_unsafe, [], {$($if_true:tt)*}, {$($if_false:tt)*}) => { $($if_false)* };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __if_unsafe {
    (, {$($if_unsafe:tt)*}, {$($if_safe:tt)*}) => { $($if_safe)* };
    (unsafe, {$($if_unsafe:tt)*}, {$($if_safe:tt)*}) => { $($if_unsafe)* };
}

#[doc(hidden)]
#[macro_export]
#[allow(unknown_lints, edition_2024_expr_fragment_specifier)]
macro_rules! __ctor_entry {
    (meta=[$($meta:meta)?], features=$features:tt, imeta=$(#[$fnmeta:meta])*, vis=[$($vis:tt)*], item=fn $ident:ident() $block:block) => {
        $crate::__support::ctor_entry!(meta=[$($meta,)?], features=$features, imeta=$(#[$fnmeta])*, vis=[$($vis)*], unsafe=, item=fn $ident() $block);
    };
    (meta=[$($meta:meta)?], features=$features:tt, imeta=$(#[$fnmeta:meta])*, vis=[$($vis:tt)*], item=unsafe fn $ident:ident() $block:block) => {
        $crate::__support::ctor_entry!(meta=[$($meta,)?], features=$features, imeta=$(#[$fnmeta])*, vis=[$($vis)*], unsafe=unsafe, item=fn $ident() $block);
    };
    (meta=[$($meta:meta)?], features=$features:tt, imeta=$(#[$fnmeta:meta])*, vis=[$($vis:tt)*], unsafe=$($unsafe:ident)?, item=fn $ident:ident() $block:block) => {
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
        $($vis)* $($unsafe)? fn $ident() {
            #[doc(hidden)]
            #[allow(unsafe_code)]
            mod __ctor_internal {
                $crate::__support::if_unsafe!($($unsafe)?, {}, {
                    $crate::__support::if_has_feature!( __warn_on_missing_unsafe, $features, {
                        #[deprecated="ctor deprecation note:\n\n \
                        Use of #[ctor] without `unsafe fn` is deprecated. As code execution before main\n\
                        is unsupported by most Rust runtime functions, these functions must be marked\n\
                        `unsafe`."]
                            const fn ctor_without_unsafe_is_deprecated() {}
                            #[allow(unused)]
                            static UNSAFE_WARNING: () = ctor_without_unsafe_is_deprecated();
                    }, {});
                });

                $crate::__support::ctor_link_section!(
                    array,
                    
                    features=$features,

                    #[allow(non_upper_case_globals, non_snake_case)]
                    #[doc(hidden)]
                    static $ident: /*unsafe*/ extern "C" fn() -> usize =
                    {
                        $crate::__support::ctor_link_section!(
                            startup,
                            
                            features=$features,

                            #[allow(non_snake_case)]
                            /*unsafe*/ extern "C" fn $ident() -> usize { unsafe { super::$ident(); 0 } }
                        );

                        $ident
                    };
                );
            }

            $block
        }
    };
    (meta=[$($meta:meta)?], features=$features:tt, imeta=$(#[$imeta:meta])*, vis=[$($vis:tt)*], item=static $ident:ident : $ty:ty = $expr:expr;) => {
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
            $crate::__support::ctor_link_section!(
                array,
                
                features=$features,

                #[allow(non_upper_case_globals, non_snake_case)]
                #[doc(hidden)]
                static $ident: /*unsafe*/ extern "C" fn() -> usize =
                {
                    $crate::__support::ctor_link_section!(
                        startup,
                        
                        features=$features,

                        #[allow(non_snake_case)]
                        /*unsafe*/ extern "C" fn $ident() -> usize { super::$ident.init_once(); 0 }
                    );

                    $ident
                };
            );
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __dtor_entry {
    (meta=[$($meta:meta)?], features=$features:tt, imeta=$(#[$fnmeta:meta])*, vis=[$($vis:tt)*], item=fn $ident:ident() $block:block) => {
        $crate::__support::dtor_entry!(meta=[$($meta,)?], features=$features, imeta=$(#[$fnmeta])*, vis=[$($vis)*], unsafe=, item=fn $ident() $block);
    };
    (meta=[$($meta:meta)?], features=$features:tt, imeta=$(#[$fnmeta:meta])*, vis=[$($vis:tt)*], item=unsafe fn $ident:ident() $block:block) => {
        $crate::__support::dtor_entry!(meta=[$($meta,)?], features=$features, imeta=$(#[$fnmeta])*, vis=[$($vis)*], unsafe=unsafe, item=fn $ident() $block);
    };
    (meta=[$($meta:meta)?], features=$features:tt, imeta=$(#[$fnmeta:meta])*, vis=[$($vis:tt)*], unsafe=$($unsafe:ident)?, item=fn $ident:ident() $block:block) => {
        $(#[$fnmeta])*
        #[allow(unused)]
        $($vis)* $($unsafe)? fn $ident() {
            #[allow(unsafe_code)]
            mod __dtor_internal {
                $crate::__support::if_unsafe!($($unsafe)?, {}, {
                    $crate::__support::if_has_feature!( __warn_on_missing_unsafe, $features, {
                        #[deprecated="dtor deprecation note:\n\n \
                        Use of #[dtor] without `unsafe fn` is deprecated. As code execution after main\n\
                        is unsupported by most Rust runtime functions, these functions must be marked\n\
                        `unsafe`."]
                        const fn dtor_without_unsafe_is_deprecated() {}
                        #[allow(unused)]
                        static UNSAFE_WARNING: () = dtor_without_unsafe_is_deprecated();
                    }, {});
                });

                $crate::__support::ctor_link_section!(
                    array,
                    
                    features=$features,

                    #[allow(non_upper_case_globals, non_snake_case)]
                    #[doc(hidden)]
                    static $ident: /*unsafe*/ extern "C" fn() -> usize =
                    {
                        $crate::__support::ctor_link_section!(
                            startup,
                            
                            features=$features,

                            #[allow(non_snake_case)]
                            /*unsafe*/ extern "C" fn $ident() -> usize { unsafe { do_atexit(__dtor); 0 } }
                        );

                        $ident
                    };
                );

                $crate::__support::ctor_link_section!(
                    exit,
                    
                    features=$features,

                    /*unsafe*/ extern "C" fn __dtor(
                        #[cfg(target_vendor = "apple")] _: *const u8
                    ) { unsafe { super::$ident() } }
                );

                #[cfg(not(target_vendor = "apple"))]
                #[inline(always)]
                pub(super) unsafe fn do_atexit(cb: unsafe extern fn()) {
                    /*unsafe*/ extern "C" {
                        fn atexit(cb: unsafe extern fn());
                    }
                    unsafe {
                        atexit(cb);
                    }
                }

                // For platforms that have __cxa_atexit, we register the dtor as scoped to dso_handle
                #[cfg(target_vendor = "apple")]
                #[inline(always)]
                pub(super) unsafe fn do_atexit(cb: /*unsafe*/ extern "C" fn(_: *const u8)) {
                    /*unsafe*/ extern "C" {
                        static __dso_handle: *const u8;
                        fn __cxa_atexit(cb: /*unsafe*/ extern "C" fn(_: *const u8), arg: *const u8, dso_handle: *const u8);
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
#[doc(hidden)]
#[macro_export]
macro_rules! __ctor_link_section {
    ($section:ident,features=$features:tt, $($block:tt)+) => {
        $crate::__support::if_has_feature!(used_linker, $features, {
            $crate::__support::ctor_link_section_attr!($section, used(linker), $($block)+);
        }, {
            $crate::__support::ctor_link_section_attr!($section, used, $($block)+);
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
#[doc(hidden)]
#[macro_export]
macro_rules! __ctor_link_section_attr {
    (array, $used:meta, $item:item) => {
        #[allow(unsafe_code)]
        $crate::__support::ctor_link_section_attr!(
            [[any(
                target_os = "linux",
                target_os = "android",
                target_os = "freebsd",
                target_os = "netbsd",
                target_os = "openbsd",
                target_os = "dragonfly",
                target_os = "illumos",
                target_os = "haiku"
            ), ".init_array"],
            [target_vendor = "apple", "__DATA,__mod_init_func"],
            [windows, ".CRT$XCU"]],
            #[$used]
            $item
        );
    };
    (startup, $used:meta, $item:item) => {
        #[cfg(not(clippy))]
        $crate::__support::ctor_link_section_attr!([[any(target_os = "linux", target_os = "android"), ".text.startup"]], $item);
        
        #[cfg(clippy)]
        $item
    };
    (exit, $used:meta, $item:item) => {
        #[cfg(not(clippy))]
        $crate::__support::ctor_link_section_attr!([[any(target_os = "linux", target_os = "android"), ".text.exit"]], $item);

        #[cfg(clippy)]
        $item
    };
    ([$( [$cond:meta, $literal:literal ] ),+], $item:item) => {
        $( #[cfg_attr($cond, link_section = $literal)] )+
        $item
    };
}
