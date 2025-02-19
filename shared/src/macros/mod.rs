/// Parse a `#[ctor]`-annotated item as if it were a proc-macro.
///
/// This macro supports both the `fn` and `static` forms of the `#[ctor]`
/// attribute, including attribute parameters.
///
/// ```rust
/// # #[cfg(any())] mod test {
/// ctor! {
///   #[ctor(link_section = "...")]
///   unsafe fn foo() { /* ... */ }
/// }
///
/// ctor! {
///   #[ctor(link_section = "...")]
///   static FOO: std::collections::HashMap<u32, String> = unsafe {
///     let mut m = std::collections::HashMap::new();
///     m.insert(1, "foo".to_string());
///     m
///   };
/// }
/// # }
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! __ctor_parse {
    ($( #[feature($fname:ident)] )* #[ctor $(($($meta:tt)*))?] $(#[$imeta:meta])* pub ( $($extra:tt)* ) $($item:tt)*) => {
        $crate::__support::unify_features!(next=$crate::__support::ctor_entry, meta=[$($($meta)*)?], features=[$($fname,)*], imeta=$(#[$imeta])*, vis=[pub($($extra)*)], item=$($item)*);
    };
    ($( #[feature($fname:ident)] )* #[ctor $(($($meta:tt)*))?] $(#[$imeta:meta])* pub $($item:tt)*) => {
        $crate::__support::unify_features!(next=$crate::__support::ctor_entry, meta=[$($($meta)*)?], features=[$($fname,)*], imeta=$(#[$imeta])*, vis=[pub], item=$($item)*);
    };
    ($( #[feature($fname:ident)] )* #[ctor $(($($meta:tt)*))?] $(#[$imeta:meta])* fn $($item:tt)*) => {
        $crate::__support::unify_features!(next=$crate::__support::ctor_entry, meta=[$($($meta)*)?], features=[$($fname,)*], imeta=$(#[$imeta])*, vis=[], item=fn $($item)*);
    };
    ($( #[feature($fname:ident)] )* #[ctor $(($($meta:tt)*))?] $(#[$imeta:meta])* unsafe $($item:tt)*) => {
        $crate::__support::unify_features!(next=$crate::__support::ctor_entry, meta=[$($($meta)*)?], features=[$($fname,)*], imeta=$(#[$imeta])*, vis=[], item=unsafe $($item)*);
    };
    ($( #[feature($fname:ident)] )* #[ctor $(($($meta:tt)*))?] $(#[$imeta:meta])* static $($item:tt)*) => {
        $crate::__support::unify_features!(next=$crate::__support::ctor_entry, meta=[$($($meta)*)?], features=[$($fname,)*], imeta=$(#[$imeta])*, vis=[], item=static $($item)*);
    };
}

/// Parse a `#[dtor]`-annotated item as if it were a proc-macro.
///
/// ```rust
/// # #[cfg(any())] mod test {
/// dtor! {
///   #[dtor]
///   unsafe fn foo() { /* ... */ }
/// }
/// # }
#[doc(hidden)]
#[macro_export]
macro_rules! __dtor_parse {
    ($( #[feature($fname:ident)] )* #[dtor $(($($meta:tt)*))?] $(#[$imeta:meta])* pub ( $($extra:tt)* ) $($item:tt)*) => {
        $crate::__support::dtor_entry!(meta=[$($($meta)*)?], features=[$($fname,)*], imeta=$(#[$imeta])*, vis=[pub($($extra)*)], item=$($item)*);
    };
    ($( #[feature($fname:ident)] )* #[dtor $(($($meta:tt)*))?] $(#[$imeta:meta])* pub $($item:tt)*) => {
        $crate::__support::dtor_entry!(meta=[$($($meta)*)?], features=[$($fname,)*], imeta=$(#[$imeta])*, vis=[pub], item=$($item)*);
    };
    ($( #[feature($fname:ident)] )* #[dtor $(($($meta:tt)*))?] $(#[$imeta:meta])* fn $($item:tt)*) => {
        $crate::__support::dtor_entry!(meta=[$($($meta)*)?], features=[$($fname,)*], imeta=$(#[$imeta])*, vis=[], item=fn $($item)*);
    };
    ($( #[feature($fname:ident)] )* #[dtor $(($($meta:tt)*))?] $(#[$imeta:meta])* unsafe $($item:tt)*) => {
        $crate::__support::dtor_entry!(meta=[$($($meta)*)?], features=[$($fname,)*], imeta=$(#[$imeta])*, vis=[], item=unsafe $($item)*);
    };
}

/// Extract #[ctor/dtor] attribute parameters and turn them into features.
///
/// Supported attributes:
///
/// - `used(linker)` -> feature: `used_linker`
/// - `link_section = ...` -> feature: `(link_section = ...)`
#[doc(hidden)]
#[macro_export]
macro_rules! __unify_features {
    (next=$next_macro:path, meta=[used(linker) $(, $($meta:tt)* )?], features=[$($features:tt,)*], $($rest:tt)*) => {
        $crate::__support::unify_features!(next=$next_macro, meta=[$($($meta)*)?], features=[used_linker,$($features,)*], $($rest)*);
    };
    (next=$next_macro:path, meta=[link_section = $section:tt $(, $($meta:tt)* )?], features=[$($features:tt,)*], $($rest:tt)*) => {
        $crate::__support::unify_features!(next=$next_macro, meta=[$($($meta)*)?], features=[(link_section=$section),$($features,)*], $($rest)*);
    };
    (next=$next_macro:path, meta=[crate_path = $path:path $(, $($meta:tt)* )?], features=[$($features:tt,)*], $($rest:tt)*) => {
        $crate::__support::unify_features!(next=$next_macro, meta=[$($($meta)*)?], features=[(crate_path=$path),$($features,)*], $($rest)*);
    };
    (next=$next_macro:path, meta=[$unknown_meta:meta $($meta:tt)*], features=[$($features:tt,)*], $($rest:tt)*) => {
        compile_error!(concat!("Unknown attribute parameter: ", stringify!($unknown_meta)));
    };
    (next=$next_macro:path, meta=[], features=$features:tt, $($rest:tt)*) => {
        $next_macro!(features=$features, $($rest)*);
    };
}

/// If the features array contains the requested feature, generates `if_true`, else `if_false`.
///
/// This macro matches the features recursively.
///
/// Example: `[(link_section = ".ctors") , used_linker , __warn_on_missing_unsafe ,]`
#[doc(hidden)]
#[macro_export]
#[allow(unknown_lints, edition_2024_expr_fragment_specifier)]
macro_rules! __if_has_feature {
    (used_linker,              [used_linker,                     $($rest:tt)*], {$($if_true:tt)*}, {$($if_false:tt)*}) => { $($if_true)* };
    (__warn_on_missing_unsafe, [__warn_on_missing_unsafe,        $($rest:tt)*], {$($if_true:tt)*}, {$($if_false:tt)*}) => { $($if_true)* };
    ((link_section(c)),        [(link_section=$section:literal), $($rest:tt)*], {$($if_true:tt)*}, {$($if_false:tt)*}) => { #[link_section = $section] $($if_true)* };

    // Fallback rules
    ($anything:tt, [$x:ident, $($rest:tt)*], {$($if_true:tt)*}, {$($if_false:tt)*}) => { $crate::__support::if_has_feature!($anything, [$($rest)*], {$($if_true)*}, {$($if_false)*}); };
    ($anything:tt, [($x:ident=$y:expr), $($rest:tt)*], {$($if_true:tt)*}, {$($if_false:tt)*}) => { $crate::__support::if_has_feature!($anything, [$($rest)*], {$($if_true)*}, {$($if_false)*}); };
    ($anything:tt, [], {$($if_true:tt)*}, {$($if_false:tt)*}) => { $($if_false)* };
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
    (features=$features:tt, imeta=$(#[$fnmeta:meta])*, vis=[$($vis:tt)*], item=unsafe fn $($item:tt)*) => {
        $crate::__support::ctor_entry!(features=$features, imeta=$(#[$fnmeta])*, vis=[$($vis)*], unsafe=unsafe, item=fn $($item)*);
    };
    (features=$features:tt, imeta=$(#[$fnmeta:meta])*, vis=[$($vis:tt)*], item=fn $($item:tt)*) => {
        $crate::__support::ctor_entry!(features=$features, imeta=$(#[$fnmeta])*, vis=[$($vis)*], unsafe=, item=fn $($item)*);
    };
    (features=$features:tt, imeta=$(#[$fnmeta:meta])*, vis=[$($vis:tt)*], item=static $ident:ident : $ty:ty = $(unsafe)? $({ $lit:literal })? $($lit2:literal)? ;) => {
        compile_error!(concat!("Use `const ", stringify!($ident), " = ", stringify!($($lit)?$($lit2)?), ";` or `static ", stringify!($ident), ": ", stringify!($ty), " = ", stringify!($($lit)?$($lit2)?), ";` instead"));
    };
    (features=$features:tt, imeta=$(#[$fnmeta:meta])*, vis=[$($vis:tt)*], item=static $ident:ident : $ty:ty = unsafe $($item:tt)*) => {
        $crate::__support::ctor_entry!(features=$features, imeta=$(#[$fnmeta])*, vis=[$($vis)*], unsafe=unsafe, item=static $ident: $ty = $($item)*);
    };
    (features=$features:tt, imeta=$(#[$fnmeta:meta])*, vis=[$($vis:tt)*], item=static $ident:ident : $ty:ty = $($item:tt)*) => {
        $crate::__support::ctor_entry!(features=$features, imeta=$(#[$fnmeta])*, vis=[$($vis)*], unsafe=, item=static $ident: $ty = $($item)*);
    };
    (features=$features:tt, imeta=$(#[$fnmeta:meta])*, vis=[$($vis:tt)*], unsafe=$($unsafe:ident)?, item=fn $ident:ident() $block:block) => {
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
            /// Internal module.
            #[doc = concat!("features=", stringify!($features))]
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
    (features=$features:tt, imeta=$(#[$imeta:meta])*, vis=[$($vis:tt)*], unsafe=$($unsafe:ident)?, item=static $ident:ident : $ty:ty = $block:block;) => {
        $(#[$imeta])*
        $($vis)* static $ident: $ident::Static<$ty> = $ident::Static::<$ty> {
            _storage: ::std::sync::OnceLock::new()
        };

        impl ::core::ops::Deref for $ident::Static<$ty> {
            type Target = $ty;
            fn deref(&self) -> &$ty {
                fn init() -> $ty $block

                self._storage.get_or_init(move || {
                    init()
                })
            }
        }

        #[doc(hidden)]
        #[allow(non_upper_case_globals, non_snake_case)]
        #[allow(unsafe_code)]
        mod $ident {
            $crate::__support::if_unsafe!($($unsafe)?, {}, {
                $crate::__support::if_has_feature!( __warn_on_missing_unsafe, $features, {
                    #[deprecated="ctor deprecation note:\n\n \
                    Use of #[ctor] without `unsafe { ... }` is deprecated. As code execution before main\n\
                    is unsupported by most Rust runtime functions, these functions must be marked\n\
                    `unsafe`."]
                        const fn ctor_without_unsafe_is_deprecated() {}
                        #[allow(unused)]
                        static UNSAFE_WARNING: () = ctor_without_unsafe_is_deprecated();
                }, {});
            });

            #[allow(non_camel_case_types, unreachable_pub)]
            pub struct Static<T> {
                pub _storage: ::std::sync::OnceLock<T>
            }

            #[cfg(target_family="wasm")]
            #[::wasm_bindgen::prelude::wasm_bindgen(start)]
            fn init() {
                _ = &*super::$ident;
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
                        /*unsafe*/ extern "C" fn $ident() -> usize { _ = &*super::$ident; 0 }
                    );

                    $ident
                };
            );
        }
    };
}

// Code note:

// You might wonder why we don't use `__attribute__((destructor))`/etc for
// dtor. Unfortunately mingw doesn't appear to properly support section-based
// hooks for shutdown, ie:

// https://github.com/Alexpux/mingw-w64/blob/d0d7f784833bbb0b2d279310ddc6afb52fe47a46/mingw-w64-crt/crt/crtdll.c

// In addition, OSX has removed support for section-based shutdown hooks after
// warning about it for a number of years:

// https://reviews.llvm.org/D45578

#[doc(hidden)]
#[macro_export]
macro_rules! __dtor_entry {
    (meta=[$($meta:meta,)*], features=$features:tt, imeta=$(#[$fnmeta:meta])*, vis=[$($vis:tt)*], item=fn $ident:ident() $block:block) => {
        $crate::__support::dtor_entry!(meta=[$($($meta,)*)?], features=$features, imeta=$(#[$fnmeta])*, vis=[$($vis)*], unsafe=, item=fn $ident() $block);
    };
    (meta=[$($meta:meta,)*], features=$features:tt, imeta=$(#[$fnmeta:meta])*, vis=[$($vis:tt)*], item=unsafe fn $ident:ident() $block:block) => {
        $crate::__support::dtor_entry!(meta=[$($($meta,)*)?], features=$features, imeta=$(#[$fnmeta])*, vis=[$($vis)*], unsafe=unsafe, item=fn $ident() $block);
    };
    (meta=[$($meta:meta,)*], features=$features:tt, imeta=$(#[$fnmeta:meta])*, vis=[$($vis:tt)*], unsafe=$($unsafe:ident)?, item=fn $ident:ident() $block:block) => {
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
    ($section:ident, features=$features:tt, $($block:tt)+) => {
        $crate::__support::if_has_feature!(used_linker, $features, {
            $crate::__support::ctor_link_section_attr!($section, $features, used(linker), $($block)+);
        }, {
            $crate::__support::ctor_link_section_attr!($section, $features, used, $($block)+);
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
    (array, $features:tt, $used:meta, $item:item) => {
        $crate::__support::if_has_feature!((link_section(c)), $features, {
            #[allow(unsafe_code)]
            #[$used]
            $item
        }, {
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
        });
    };
    (startup, $features:tt, $used:meta, $item:item) => {
        #[cfg(not(clippy))]
        $crate::__support::ctor_link_section_attr!([[any(target_os = "linux", target_os = "android"), ".text.startup"]], $item);

        #[cfg(clippy)]
        $item
    };
    (exit, $features:tt, $used:meta, $item:item) => {
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
