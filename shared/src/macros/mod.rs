//! Shared macros for the `ctor` and `dtor` crates.

#[doc(hidden)]
#[allow(unused)]
pub(crate) mod __support {
    /// Return type for the constructor. Why is this needed?
    ///
    /// On Windows, `.CRT$XIA` … `.CRT$XIZ` constructors are required to return a `usize` value. We don't know
    /// if the user is putting this function into a retval-requiring section or a non-retval section, so we
    /// just return a `usize` value which is always valid and just ignored if not needed.
    ///
    /// Miri is pedantic about this, so we just return `()` if we're running under miri.
    ///
    /// See <https://learn.microsoft.com/en-us/cpp/c-runtime-library/reference/initterm-initterm-e?view=msvc-170>
    #[cfg(all(windows, not(miri)))]
    pub type CtorRetType = usize;
    #[cfg(any(not(windows), miri))]
    pub type CtorRetType = ();

    pub use crate::__ctor_call as ctor_call;
    pub use crate::__ctor_entry as ctor_entry;
    pub use crate::__ctor_link_section as ctor_link_section;
    pub use crate::__ctor_link_section_attr as ctor_link_section_attr;
    pub use crate::__ctor_parse as ctor_parse;
    pub use crate::__dtor_entry as dtor_entry;
    pub use crate::__dtor_parse_impl as dtor_parse_impl;
    pub use crate::__get_priority as get_priority;
    pub use crate::__if_has_feature as if_has_feature;
    pub use crate::__if_unsafe as if_unsafe;
    pub use crate::__priority_to_literal as priority_to_literal;
    pub use crate::__unify_features as unify_features;

    #[cfg(all(feature = "priority", target_vendor = "apple"))]
    pub use ::link_section;

    /// Define a link section when using the priority parameter on Apple
    /// targets.
    #[cfg(all(feature = "priority", target_vendor = "apple"))]
    #[doc(hidden)]
    pub mod explicit_ctor {
        link_section::declarative::section!(
            #[section]
            pub static CTOR: link_section::TypedSection<(fn(), u16)>;
        );

        crate::__support::ctor_call!(features = [], {
            // SAFETY: The CTOR section is only accessed in this function, and
            // this function is only ever called once.
            #[allow(unsafe_code)]
            unsafe {
                CTOR.as_mut_slice()
                    .sort_unstable_by_key(|(_, priority)| *priority);
            }
            for (ctor, _) in CTOR {
                ctor();
            }
        });
    }
}

/// Parse a `#[ctor]`-annotated item as if it were a proc-macro.
///
/// This macro supports both the `fn` and `static` forms of the `#[ctor]`
/// attribute, including attribute parameters.
///
/// ```rust
/// # #[cfg(any())] // disabled due to code sharing between ctor/dtor
/// # mod test {
/// # use ctor::declarative::ctor;
/// ctor! {
///   /// Create a ctor with a link section
/// # #[cfg(any())]
///   #[ctor(link_section = ".ctors")]
///   unsafe fn foo() { /* ... */ }
/// }
///
/// ctor! {
///   #[ctor]
/// # #[cfg(any())]
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
    (#[ctor $(($($meta:tt)*))?] $(#[$imeta:meta])* pub ( $($extra:tt)* ) $($item:tt)*) => {
        $crate::__support::unify_features!(next=$crate::__support::ctor_entry, meta=[$($($meta)*)?], imeta=$(#[$imeta])*, vis=[pub($($extra)*)], item=$($item)*);
    };
    (#[ctor $(($($meta:tt)*))?] $(#[$imeta:meta])* pub $($item:tt)*) => {
        $crate::__support::unify_features!(next=$crate::__support::ctor_entry, meta=[$($($meta)*)?], imeta=$(#[$imeta])*, vis=[pub], item=$($item)*);
    };
    (#[ctor $(($($meta:tt)*))?] $(#[$imeta:meta])* fn $($item:tt)*) => {
        $crate::__support::unify_features!(next=$crate::__support::ctor_entry, meta=[$($($meta)*)?], imeta=$(#[$imeta])*, vis=[], item=fn $($item)*);
    };
    (#[ctor $(($($meta:tt)*))?] $(#[$imeta:meta])* unsafe $($item:tt)*) => {
        $crate::__support::unify_features!(next=$crate::__support::ctor_entry, meta=[$($($meta)*)?], imeta=$(#[$imeta])*, vis=[], item=unsafe $($item)*);
    };
    (#[ctor $(($($meta:tt)*))?] $(#[$imeta:meta])* static $($item:tt)*) => {
        $crate::__support::unify_features!(next=$crate::__support::ctor_entry, meta=[$($($meta)*)?], imeta=$(#[$imeta])*, vis=[], item=static $($item)*);
    };
    // Reorder attributes that aren't `#[ctor]`
    (#[$imeta:meta] $($rest:tt)*) => {
        $crate::__support::ctor_parse!(__reorder__(#[$imeta],), $($rest)*);
    };
    (__reorder__($(#[$imeta:meta],)*), #[ctor $(($($meta:tt)*))?] $($rest:tt)*) => {
        $crate::__support::ctor_parse!(#[ctor $(($($meta)*))?] $(#[$imeta])* $($rest)*);
    };
    (__reorder__($(#[$imeta:meta],)*), #[$imeta2:meta] $($rest:tt)*) => {
        $crate::__support::ctor_parse!(__reorder__($(#[$imeta],)*#[$imeta2],), $($rest)*);
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
macro_rules! __dtor_parse_impl {
    (#[dtor $(($($meta:tt)*))?] $(#[$imeta:meta])* pub ( $($extra:tt)* ) $($item:tt)*) => {
        $crate::__support::unify_features!(next=$crate::__support::dtor_entry, meta=[$($($meta)*)?], imeta=$(#[$imeta])*, vis=[pub($($extra)*)], item=$($item)*);
    };
    (#[dtor $(($($meta:tt)*))?] $(#[$imeta:meta])* pub $($item:tt)*) => {
        $crate::__support::unify_features!(next=$crate::__support::dtor_entry, meta=[$($($meta)*)?], imeta=$(#[$imeta])*, vis=[pub], item=$($item)*);
    };
    (#[dtor $(($($meta:tt)*))?] $(#[$imeta:meta])* fn $($item:tt)*) => {
        $crate::__support::unify_features!(next=$crate::__support::dtor_entry, meta=[$($($meta)*)?], imeta=$(#[$imeta])*, vis=[], item=fn $($item)*);
    };
    (#[dtor $(($($meta:tt)*))?] $(#[$imeta:meta])* unsafe $($item:tt)*) => {
        $crate::__support::unify_features!(next=$crate::__support::dtor_entry, meta=[$($($meta)*)?], imeta=$(#[$imeta])*, vis=[], item=unsafe $($item)*);
    };
    // Reorder attributes that aren't `#[dtor]`
    (#[$imeta:meta] $($rest:tt)*) => {
        $crate::__support::dtor_parse_impl!(__reorder__(#[$imeta],), $($rest)*);
    };
    (__reorder__($(#[$imeta:meta],)*), #[dtor $(($($meta:tt)*))?] $($rest:tt)*) => {
        $crate::__support::dtor_parse_impl!(#[dtor $(($($meta)*))?] $(#[$imeta])* $($rest)*);
    };
    (__reorder__($(#[$imeta:meta],)*), #[$imeta2:meta] $($rest:tt)*) => {
        $crate::__support::dtor_parse_impl!(__reorder__($(#[$imeta],)*#[$imeta2],), $($rest)*);
    };
}

/// A macro that generates the appropriate feature extraction macros.
macro_rules! declare_features {
    ( $(#[doc = $doc1:literal])* crate = $crate_features:tt; $(#[doc = $doc2:literal])* attr = $attrs:tt; ) => {
        declare_features!( __ crate $crate_features );
    };

    ( __ crate [$(
        $( #[doc = $doc:literal] )*
        $feature_name:ident $feature_name_str:literal = $feature_include_macro:ident ;
    )*] ) => {
        /// # Crate features
        ///
        $(
            #[doc = concat!("<code>", stringify!($feature_name), "</code>: ")]
            $( #[doc = $doc] )*
            #[doc = "\n"]
        )*
        pub mod features {
        }

        $(
        #[doc(hidden)]
        #[macro_export]
        #[cfg(feature = $feature_name_str)]
        macro_rules! $feature_include_macro {
            ($true:item $false:item) => {
                $true
            };
        }

        #[doc(hidden)]
        #[macro_export]
        #[cfg(not(feature = $feature_name_str))]
        macro_rules! $feature_include_macro {
            ($true:item $false:item) => {
                $false
            };
        }
        )*

        #[doc(hidden)]
        #[macro_export]
        macro_rules! __features_expand {
            (next=$next_macro:path, $args:tt) => {
                $crate::__features_expand_all!(next=$next_macro, $args, $( $feature_name / $feature_include_macro )*);
            };
        }
    };
}

declare_features!(
    /// Crate features: name/name as string/include macro.
    crate = [
        /// Enable support for the standard library. This is required for static ctor variables, but not for functions.
        std "std" = __include_std_feature;
        /// Mark all ctor functions with `used(linker)`.
        used_linker "used_linker" = __include_used_linker_feature;
        /// Enable support for the proc-macro `#[ctor]` and `#[dtor]` attributes.
        proc_macro "proc_macro" = __include_proc_macro_feature;
        /// Do not warn when a ctor or dtor is missing the `unsafe` keyword.
        no_warn_on_missing_unsafe "no_warn_on_missing_unsafe" = __include_no_warn_on_missing_unsafe_feature;
        /// Enable support for the priority parameter.
        priority "priority" = __include_priority_feature;
    ];

    /// Attributes.
    attr = [
        /// Marks a ctor/dtor as unsafe. This will become a warning in 1.0.
        unsafe = [unsafe];
        /// Place the initialization function pointer in a custom link section. This may cause the initialization function
        /// to fail to run or run earlier or later than other `ctor` functions.
        link_section = [link_section($section:literal)];
        /// Specify a custom crate path for the `ctor` crate. Used when re-exporting the ctor macro.
        crate_path = [crate_path = $path:path];
        /// Make the ctor function anonymous.
        anonymous = [anonymous];
        /// Mark this function with `used(linker)`.
        used_linker = [used(linker)];
        /// Set the ctor priority to a given value.
        priority = [priority = $priority:literal];
    ];
);

/// Expands all of the crate features into the features list.
#[macro_export]
#[doc(hidden)]
macro_rules! __features_expand_all {
    // Entry
    (next=$next:path, $args:tt, $($macro:tt)*) => {
        $crate::__features_expand_all!(: [] $next, $args, $($macro)*);
    };
    (: [$($features:tt)*] $next:path, $args:tt, $first_macro_feature:ident / $first_macro:ident $($rest:tt)*) => {
        $crate:: $first_macro !(
            $crate::__features_expand_all!(: [$first_macro_feature,$($features)*] $next, $args, $($rest)*);
            $crate::__features_expand_all!(: [$($features)*] $next, $args, $($rest)*);
        );
    };
    // Exit
    (: [$($features:tt)*] $next:path, $args:tt,) => {
        $next!(features=[$($features)*], $args);
    };
}

/// Extract #[ctor/dtor] attribute parameters and crate features and turn them
/// into a unified feature array.
///
/// Supported attributes:
///
/// - `used(linker)` -> crate feature: `used_linker`
/// - `std` -> crate feature: `std`
/// - `link_section = ...` -> feature: `(link_section = ...)`
/// - `crate_path = ...` -> feature: `(crate_path = ...)`
#[doc(hidden)]
#[macro_export]
macro_rules! __unify_features {
    // Entry
    (next=$next_macro:path, meta=[$($meta:tt)*], $($rest:tt)*) => {
        // Expand all crate features
        $crate::__features_expand!(next=$crate::__unify_features, [next=$next_macro, meta=[$($meta)*], $($rest)*]);
    };

    // Post-expansion
    (features=[$($features:tt)*], [next=$next_macro:path, meta=[$($meta:tt)*], $($rest:tt)*]) => {
        $crate::__support::unify_features!(continue, next=$next_macro, meta=[$($meta)*], features=[$($features)*], $($rest)*);
    };

    // Parse meta into features
    (continue, next=$next_macro:path, meta=[unsafe $(, $($meta:tt)* )?], features=[$($features:tt)*], $($rest:tt)*) => {
        $crate::__support::unify_features!(continue, next=$next_macro, meta=[$($($meta)*)?], features=[no_warn_on_missing_unsafe,$($features)*], $($rest)*);
    };
    (continue, next=$next_macro:path, meta=[used(linker) $(, $($meta:tt)* )?], features=[$($features:tt)*], $($rest:tt)*) => {
        $crate::__support::unify_features!(continue, next=$next_macro, meta=[$($($meta)*)?], features=[used_linker,$($features)*], $($rest)*);
    };
    (continue, next=$next_macro:path, meta=[link_section = $section:tt $(, $($meta:tt)* )?], features=[$($features:tt)*], $($rest:tt)*) => {
        $crate::__support::unify_features!(continue, next=$next_macro, meta=[$($($meta)*)?], features=[(link_section=($section)),$($features)*], $($rest)*);
    };
    (continue, next=$next_macro:path, meta=[crate_path = $path:path $(, $($meta:tt)* )?], features=[$($features:tt)*], $($rest:tt)*) => {
        $crate::__support::unify_features!(continue, next=$next_macro, meta=[$($($meta)*)?], features=[(crate_path=$path),$($features)*], $($rest)*);
    };
    (continue, next=$next_macro:path, meta=[anonymous $(, $($meta:tt)* )?], features=[$($features:tt)*], $($rest:tt)*) => {
        $crate::__support::unify_features!(continue, next=$next_macro, meta=[$($($meta)*)?], features=[anonymous,$($features)*], $($rest)*);
    };
    (continue, next=$next_macro:path, meta=[priority = $priority:tt $(, $($meta:tt)* )?], features=[$($features:tt)*], $($rest:tt)*) => {
        // Convert the priority to a literal
        $crate::__support::priority_to_literal!($crate::__support::unify_features, (@priority next=$next_macro, meta=[$($($meta)*)?], features=[$($features)*], $($rest)*), $priority);
    };
    ((@priority next=$next_macro:path, meta=[$($meta:tt)*], features=[$($features:tt)*], $($rest:tt)*), $priority:tt) => {
        $crate::__support::unify_features!(continue, next=$next_macro, meta=[$($($meta)*)?], features=[(priority=($priority)),$($features)*], $($rest)*);
    };
    (continue, next=$next_macro:path, meta=[$unknown_meta:meta $($meta:tt)*], features=[$($features:tt)*], $($rest:tt)*) => {
        compile_error!(concat!("Unknown attribute parameter: ", stringify!($unknown_meta)));
    };

    (continue, next=$next_macro:path, meta=[], features=[$($features:tt)*], $($rest:tt)*) => {
        $next_macro!(features=[$($features)*], $($rest)*);
    };
}

/// If the features array contains the requested feature, generates `if_true`, else `if_false`.
///
/// This macro matches the features recursively.
///
/// Example: `[(link_section = ".ctors") , used_linker , no_warn_on_missing_unsafe ,]`
#[doc(hidden)]
#[macro_export]
#[allow(unknown_lints, edition_2024_expr_fragment_specifier)]
macro_rules! __if_has_feature {
    (std,                         [std,                               $($rest:tt)*], {$($if_true:tt)*}, {$($if_false:tt)*}) => { $($if_true)* };
    (used_linker,                 [used_linker,                       $($rest:tt)*], {$($if_true:tt)*}, {$($if_false:tt)*}) => { $($if_true)* };
    (no_warn_on_missing_unsafe,   [no_warn_on_missing_unsafe,         $($rest:tt)*], {$($if_true:tt)*}, {$($if_false:tt)*}) => { $($if_true)* };
    (anonymous,                   [anonymous,                         $($rest:tt)*], {$($if_true:tt)*}, {$($if_false:tt)*}) => { $($if_true)* };
    ((link_section(c)),           [(link_section=($section:literal)), $($rest:tt)*], {$($if_true:tt)*}, {$($if_false:tt)*}) => { #[link_section = $section] $($if_true)* };
    ((priority(p)),               [(priority=($priority:literal)),    $($rest:tt)*], {$($if_true:tt)*}, {$($if_false:tt)*}) => { $($if_true)* };

    // Fallback rules
    ($anything:tt, [$x:ident, $($rest:tt)*], {$($if_true:tt)*}, {$($if_false:tt)*}) => { $crate::__support::if_has_feature!($anything, [$($rest)*], {$($if_true)*}, {$($if_false)*}); };
    ($anything:tt, [($x:ident=$y:tt), $($rest:tt)*], {$($if_true:tt)*}, {$($if_false:tt)*}) => { $crate::__support::if_has_feature!($anything, [$($rest)*], {$($if_true)*}, {$($if_false)*}); };
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
macro_rules! __get_priority {
    ($next:path, $args:tt, [(priority=($priority:literal)), $($rest:tt)*]) => { $next!($args, (".", $priority)); };
    ($next:path, $args:tt, [$x:ident, $($rest:tt)*])                       => { $crate::__support::get_priority!($next, $args, [$($rest)*]); };
    ($next:path, $args:tt, [($x:ident=$y:tt), $($rest:tt)*])               => { $crate::__support::get_priority!($next, $args, [$($rest)*]); };
    ($next:path, $args:tt, [])                                             => { $next!($args, ("")); };
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
        $crate::__support::if_has_feature!(anonymous, $features, {
            $crate::__support::ctor_entry!(unnamed, features=$features, imeta=$(#[$fnmeta])*, vis=[$($vis)*], unsafe=$($unsafe)?, item=fn $ident() $block);
        }, {
            // AIX requires special handling
            #[cfg(target_os = "aix")]
            $crate::__support::ctor_entry!(aix, features=$features, imeta=$(#[$fnmeta])*, vis=[$($vis)*], unsafe=$($unsafe)?, item=fn $ident() $block);
            
            #[cfg(not(target_os = "aix"))]
            $crate::__support::ctor_entry!(named, features=$features, imeta=$(#[$fnmeta])*, vis=[$($vis)*], unsafe=$($unsafe)?, item=fn $ident() $block);
        });
    };
    (unnamed,features=$features:tt, imeta=$(#[$fnmeta:meta])*, vis=[$($vis:tt)*], unsafe=$($unsafe:ident)?, item=fn $ident:ident() $block:block) => {
        const _: () = {
            #[cfg(target_os = "aix")]
            $crate::__support::ctor_entry!(aix, features=$features, imeta=$(#[$fnmeta])*, vis=[$($vis)*], unsafe=$($unsafe)?, item=fn $ident() $block);
            
            #[cfg(not(target_os = "aix"))]
            $crate::__support::ctor_entry!(named, features=$features, imeta=$(#[$fnmeta])*, vis=[$($vis)*], unsafe=$($unsafe)?, item=fn $ident() $block);
        };
    };
    (aix, features=$features:tt, imeta=$(#[$fnmeta:meta])*, vis=[$($vis:tt)*], unsafe=$($unsafe:ident)?, item=fn $ident:ident() $block:block) => {
        $(#[$fnmeta])*
        #[allow(unused)]
        $($vis)* $($unsafe)? fn $ident() $block

        // AIX-specific constructor wrapper using __sinit naming convention
        // The function name must start with __sinit80000000_ for AIX to recognize it as a constructor
        // We wrap in a const block to create a unique namespace and avoid module name collisions
        const _: () = {
            #[unsafe(no_mangle)]
            #[export_name = concat!("__sinit80000000_", module_path!(), "_", stringify!($ident), "_L", line!(), "C", column!())]
            extern "C" fn aix_ctor_wrapper() {
                #[allow(unsafe_code)]
                {
                    $crate::__support::if_unsafe!($($unsafe)?, {}, {
                        $crate::__support::if_has_feature!( __no_warn_on_missing_unsafe, $features, {
                            #[deprecated="ctor deprecation note:\n\n \
                            Use of #[ctor] without `unsafe fn` is deprecated. As code execution before main\n\
                            is unsupported by most Rust runtime functions, these functions must be marked\n\
                            `unsafe`."]
                                const fn ctor_without_unsafe_is_deprecated() {}
                                #[allow(unused)]
                                static UNSAFE_WARNING: () = ctor_without_unsafe_is_deprecated();
                        }, {});
                    });

                    unsafe { $ident(); }
                }
            }
        };
    };
    (named, features=$features:tt, imeta=$(#[$fnmeta:meta])*, vis=[$($vis:tt)*], unsafe=$($unsafe:ident)?, item=fn $ident:ident() $block:block) => {
        $(#[$fnmeta])*
        #[allow(unused)]
        $($vis)* $($unsafe)? fn $ident() {
            #[allow(unsafe_code)]
            {
                $crate::__support::if_unsafe!($($unsafe)?, {}, {
                    $crate::__support::if_has_feature!( no_warn_on_missing_unsafe, $features, {}, {
                        #[deprecated="ctor deprecation note:\n\n\
                        Use of #[ctor] without `#[ctor(unsafe)]` or `unsafe fn` is deprecated. As code execution\n\
                        before main is unsupported by most Rust runtime functions, these functions must be marked\n\
                        `unsafe`."]
                            const fn ctor_without_unsafe_is_deprecated() {}
                            #[allow(unused)]
                            static UNSAFE_WARNING: () = ctor_without_unsafe_is_deprecated();
                    });
                });

                $crate::__support::ctor_call!(
                    features=$features,
                    { unsafe { $ident(); } }
                );
            }

            #[cfg(target_family = "wasm")]
            {
                static __CTOR__INITIALIZED: ::core::sync::atomic::AtomicBool = ::core::sync::atomic::AtomicBool::new(false);
                if __CTOR__INITIALIZED.swap(true, ::core::sync::atomic::Ordering::Relaxed) {
                    return;
                }
            }

            $block
        }
    };
    (features=$features:tt, imeta=$(#[$imeta:meta])*, vis=[$($vis:tt)*], unsafe=$($unsafe:ident)?, item=static $ident:ident : $ty:ty = $block:block;) => {
        $crate::__support::if_has_feature!(std, $features, {
            #[allow(clippy::incompatible_msrv)] // MSRV for statics is 1.70
            $(#[$imeta])*
            $($vis)* static $ident: $ident::Static<$ty> = $ident::Static::<$ty> {
                _storage: {
                    $crate::__support::ctor_call!(
                        features=$features,
                        { _ = &*$ident; }
                    );

                    ::std::sync::OnceLock::new()
                }
            };

            #[allow(clippy::incompatible_msrv)] // MSRV for statics is 1.70
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
                    $crate::__support::if_has_feature!( no_warn_on_missing_unsafe, $features, {}, {
                        #[deprecated="ctor deprecation note:\n\n\
                        Use of #[ctor] without `#[ctor(unsafe)]` is deprecated. As code execution before main\n\
                        is unsupported by most Rust runtime functions, these functions must be marked\n\
                        `unsafe`."]
                            const fn ctor_without_unsafe_is_deprecated() {}
                            #[allow(unused)]
                            static UNSAFE_WARNING: () = ctor_without_unsafe_is_deprecated();
                    });
                });

                #[allow(non_camel_case_types, unreachable_pub)]
                pub struct Static<T> {
                    #[allow(clippy::incompatible_msrv)] // MSRV for statics is 1.70
                    pub _storage: ::std::sync::OnceLock<T>
                }
            }
        }, {
            compile_error!("`#[ctor]` on `static` items requires the `std` feature");
        });
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
    (features=$features:tt, imeta=$(#[$fnmeta:meta])*, vis=[$($vis:tt)*], item=fn $ident:ident() $block:block) => {
        $crate::__support::dtor_entry!(features=$features, imeta=$(#[$fnmeta])*, vis=[$($vis)*], unsafe=, item=fn $ident() $block);
    };
    (features=$features:tt, imeta=$(#[$fnmeta:meta])*, vis=[$($vis:tt)*], item=unsafe fn $ident:ident() $block:block) => {
        $crate::__support::dtor_entry!(features=$features, imeta=$(#[$fnmeta])*, vis=[$($vis)*], unsafe=unsafe, item=fn $ident() $block);
    };
    (features=$features:tt, imeta=$(#[$fnmeta:meta])*, vis=[$($vis:tt)*], unsafe=$($unsafe:ident)?, item=fn $ident:ident() $block:block) => {
        $crate::__support::if_has_feature!(anonymous, $features, {
            $crate::__support::dtor_entry!(unnamed, features=$features, imeta=$(#[$fnmeta])*, vis=[$($vis)*], unsafe=$($unsafe)?, item=fn $ident() $block);
        }, {
            $crate::__support::dtor_entry!(named, features=$features, imeta=$(#[$fnmeta])*, vis=[$($vis)*], unsafe=$($unsafe)?, item=fn $ident() $block);
        });
    };
    (unnamed, features=$features:tt, imeta=$(#[$fnmeta:meta])*, vis=[$($vis:tt)*], unsafe=$($unsafe:ident)?, item=fn $ident:ident() $block:block) => {
        const _: () = {
            $crate::__support::dtor_entry!(named, features=$features, imeta=$(#[$fnmeta])*, vis=[$($vis)*], unsafe=$($unsafe)?, item=fn $ident() $block);
        };
    };
    (named, features=$features:tt, imeta=$(#[$fnmeta:meta])*, vis=[$($vis:tt)*], unsafe=$($unsafe:ident)?, item=fn $ident:ident() $block:block) => {
        $(#[$fnmeta])*
        #[allow(unused)]
        $($vis)* $($unsafe)? fn $ident() {
            #[allow(unsafe_code)]
            {
                $crate::__support::if_unsafe!($($unsafe)?, {}, {
                    $crate::__support::if_has_feature!( no_warn_on_missing_unsafe, $features, {}, {
                        #[deprecated="dtor deprecation note:\n\n\
                        Use of #[dtor] without `#[dtor(unsafe)]` or `unsafe fn` is deprecated. As code execution\n\
                        before main is unsupported by most Rust runtime functions, these functions must be marked\n\
                        `unsafe`."]
                        const fn dtor_without_unsafe_is_deprecated() {}
                        #[allow(unused)]
                        static UNSAFE_WARNING: () = dtor_without_unsafe_is_deprecated();
                    });
                });

                $crate::__support::ctor_call!(
                    features=$features,
                    { unsafe {
                        // Maintain compatibility with earlier versions of dtor. For
                        // ctor 1.0 we can remove `dtor` as a default and then make
                        // this `at_library_exit` by default.
                        #[cfg(target_vendor = "apple")]
                        $crate::__support::at_library_exit(__dtor);
                        #[cfg(not(target_vendor = "apple"))]
                        $crate::__support::at_binary_exit(__dtor);
                    } }
                );

                $crate::__support::ctor_link_section!(
                    exit,
                    features=$features,
                    (""),

                    /*unsafe*/ extern "C" fn __dtor() { unsafe { $ident() } }
                );
            }

            $block
        }
    };
}

/// Annotate a block with its appropriate link section.
#[doc(hidden)]
#[macro_export]
macro_rules! __ctor_call {
    (features=$features:tt, { $($block:tt)+ } ) => {
        $crate::__support::get_priority!($crate::__support::ctor_call, [features=$features, { $($block)+ }], $features);
    };
    ([features=$features:tt, { $($block:tt)+ }], ("")) => {
        $crate::__support::ctor_call!(@next [features=$features, { $($block)+ }], (""));
    };
    ([features=$features:tt, { $($block:tt)+ }], (".", $priority:literal)) => {
        #[cfg(target_vendor = "apple")]
        $crate::__support::link_section::declarative::in_section!(
            #[in_section($crate::__support::explicit_ctor::CTOR)]
            static _: (fn(), u16) = ({ fn ctor() { $($block)+ }; ctor }, $priority);
        );
        #[cfg(not(target_vendor = "apple"))]
        $crate::__support::ctor_call!(@next [features=$features, { $($block)+ }], (".", $priority));
    };
    (@next [features=$features:tt, { $($block:tt)+ }], $priority:tt) => {
        $crate::__support::ctor_link_section!(
            array,
            features=$features,
            $priority,

            #[allow(non_upper_case_globals, non_snake_case)]
            #[doc(hidden)]
            static __CTOR_FUNCTION: /*unsafe*/ extern "C" fn() -> $crate::__support::CtorRetType =
            {
                $crate::__support::ctor_link_section!(
                    startup,
                    features=$features,
                    (""),

                    #[allow(non_snake_case)]
                    /*unsafe*/ extern "C" fn __CTOR_FUNCTION_INNER() -> $crate::__support::CtorRetType {
                        $($block)+;
                        ::core::default::Default::default()
                    }
                );

                __CTOR_FUNCTION_INNER
            };
        );
    }
}

/// Annotate a block with its appropriate link section.
#[doc(hidden)]
#[macro_export]
macro_rules! __ctor_link_section {
    ($section:ident, features=$features:tt, $priority:tt, $($block:tt)+) => {
        $crate::__support::if_has_feature!(used_linker, $features, {
            $crate::__support::ctor_link_section_attr!($section, $features, used(linker), $priority, $($block)+);
        }, {
            $crate::__support::ctor_link_section_attr!($section, $features, used, $priority, $($block)+);
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
            target_os = "aix",
            target_vendor = "apple",
            target_family = "wasm",
            target_arch = "xtensa",
            target_vendor = "pc"
        )))]
        compile_error!("#[ctor]/#[dtor] is not supported on the current target");
    }
}

/// Apply either the default cfg-based link section attributes, or
/// the overridden link_section attribute.
#[doc(hidden)]
#[macro_export]
macro_rules! __ctor_link_section_attr {
    (array, $features:tt, $used:meta, ($($priority:tt)*), $item:item) => {
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
                    target_os = "haiku",
                    target_family = "wasm"
                ), (concat!(".init_array", $($priority)*))],
                [target_arch = "xtensa", (concat!(".ctors", $($priority)*))],
                // macOS/Darwin do not support the priority parameter in the link section
                [target_vendor = "apple", (concat!("__DATA,__mod_init_func,mod_init_funcs"))],
                [all(target_vendor = "pc", any(target_env = "gnu", target_env = "msvc")), (concat!(".CRT$XCU", $($priority)*))],
                // cygwin support: rustc 1.85 does not like the explicit target_os = "cygwin" condition (https://github.com/mmastrac/rust-ctor/issues/356)
                // We can work around this by excluding gnu and msvc target envs
                [all(target_vendor = "pc", not(any(target_env = "gnu", target_env = "msvc"))), (concat!(".ctors", $($priority)*))]
                ],
                #[$used]
                $item
            );
        });
    };
    (startup, $features:tt, $used:meta, $priority:tt, $item:item) => {
        #[cfg(not(clippy))]
        $crate::__support::ctor_link_section_attr!([[any(target_os = "linux", target_os = "android"), (".text.startup")]], $item);

        #[cfg(clippy)]
        $item
    };
    (exit, $features:tt, $used:meta, $priority:tt, $item:item) => {
        #[cfg(not(clippy))]
        $crate::__support::ctor_link_section_attr!([[any(target_os = "linux", target_os = "android"), (".text.exit")]], $item);

        #[cfg(clippy)]
        $item
    };
    ([$( [$cond:meta, ($($literal:tt)*) ] ),+], $item:item) => {
        #[cfg(not(clippy))]
        $( #[cfg_attr($cond, link_section = $($literal)*)] )+
        $item

        #[cfg(clippy)]
        $item
    };
}

#[cfg(target_vendor = "apple")]
#[doc(hidden)]
#[macro_export]
macro_rules! __priority_to_literal {
    ($n:path,$a:tt,$priority:literal) => {
        $n!($a, $priority);
    };
}

#[cfg(not(target_vendor = "apple"))]
#[doc(hidden)]
#[macro_export]
macro_rules! __priority_to_literal {
    ($n:path,$a:tt,0) => {
        $n!($a, "000");
    };
    ($n:path,$a:tt,1) => {
        $n!($a, "001");
    };
    ($n:path,$a:tt,2) => {
        $n!($a, "002");
    };
    ($n:path,$a:tt,3) => {
        $n!($a, "003");
    };
    ($n:path,$a:tt,4) => {
        $n!($a, "004");
    };
    ($n:path,$a:tt,5) => {
        $n!($a, "005");
    };
    ($n:path,$a:tt,6) => {
        $n!($a, "006");
    };
    ($n:path,$a:tt,7) => {
        $n!($a, "007");
    };
    ($n:path,$a:tt,8) => {
        $n!($a, "008");
    };
    ($n:path,$a:tt,9) => {
        $n!($a, "009");
    };
    ($n:path,$a:tt,10) => {
        $n!($a, "010");
    };
    ($n:path,$a:tt,11) => {
        $n!($a, "011");
    };
    ($n:path,$a:tt,12) => {
        $n!($a, "012");
    };
    ($n:path,$a:tt,13) => {
        $n!($a, "013");
    };
    ($n:path,$a:tt,14) => {
        $n!($a, "014");
    };
    ($n:path,$a:tt,15) => {
        $n!($a, "015");
    };
    ($n:path,$a:tt,16) => {
        $n!($a, "016");
    };
    ($n:path,$a:tt,17) => {
        $n!($a, "017");
    };
    ($n:path,$a:tt,18) => {
        $n!($a, "018");
    };
    ($n:path,$a:tt,19) => {
        $n!($a, "019");
    };
    ($n:path,$a:tt,20) => {
        $n!($a, "020");
    };
    ($n:path,$a:tt,21) => {
        $n!($a, "021");
    };
    ($n:path,$a:tt,22) => {
        $n!($a, "022");
    };
    ($n:path,$a:tt,23) => {
        $n!($a, "023");
    };
    ($n:path,$a:tt,24) => {
        $n!($a, "024");
    };
    ($n:path,$a:tt,25) => {
        $n!($a, "025");
    };
    ($n:path,$a:tt,26) => {
        $n!($a, "026");
    };
    ($n:path,$a:tt,27) => {
        $n!($a, "027");
    };
    ($n:path,$a:tt,28) => {
        $n!($a, "028");
    };
    ($n:path,$a:tt,29) => {
        $n!($a, "029");
    };
    ($n:path,$a:tt,30) => {
        $n!($a, "030");
    };
    ($n:path,$a:tt,31) => {
        $n!($a, "031");
    };
    ($n:path,$a:tt,32) => {
        $n!($a, "032");
    };
    ($n:path,$a:tt,33) => {
        $n!($a, "033");
    };
    ($n:path,$a:tt,34) => {
        $n!($a, "034");
    };
    ($n:path,$a:tt,35) => {
        $n!($a, "035");
    };
    ($n:path,$a:tt,36) => {
        $n!($a, "036");
    };
    ($n:path,$a:tt,37) => {
        $n!($a, "037");
    };
    ($n:path,$a:tt,38) => {
        $n!($a, "038");
    };
    ($n:path,$a:tt,39) => {
        $n!($a, "039");
    };
    ($n:path,$a:tt,40) => {
        $n!($a, "040");
    };
    ($n:path,$a:tt,41) => {
        $n!($a, "041");
    };
    ($n:path,$a:tt,42) => {
        $n!($a, "042");
    };
    ($n:path,$a:tt,43) => {
        $n!($a, "043");
    };
    ($n:path,$a:tt,44) => {
        $n!($a, "044");
    };
    ($n:path,$a:tt,45) => {
        $n!($a, "045");
    };
    ($n:path,$a:tt,46) => {
        $n!($a, "046");
    };
    ($n:path,$a:tt,47) => {
        $n!($a, "047");
    };
    ($n:path,$a:tt,48) => {
        $n!($a, "048");
    };
    ($n:path,$a:tt,49) => {
        $n!($a, "049");
    };
    ($n:path,$a:tt,50) => {
        $n!($a, "050");
    };
    ($n:path,$a:tt,51) => {
        $n!($a, "051");
    };
    ($n:path,$a:tt,52) => {
        $n!($a, "052");
    };
    ($n:path,$a:tt,53) => {
        $n!($a, "053");
    };
    ($n:path,$a:tt,54) => {
        $n!($a, "054");
    };
    ($n:path,$a:tt,55) => {
        $n!($a, "055");
    };
    ($n:path,$a:tt,56) => {
        $n!($a, "056");
    };
    ($n:path,$a:tt,57) => {
        $n!($a, "057");
    };
    ($n:path,$a:tt,58) => {
        $n!($a, "058");
    };
    ($n:path,$a:tt,59) => {
        $n!($a, "059");
    };
    ($n:path,$a:tt,60) => {
        $n!($a, "060");
    };
    ($n:path,$a:tt,61) => {
        $n!($a, "061");
    };
    ($n:path,$a:tt,62) => {
        $n!($a, "062");
    };
    ($n:path,$a:tt,63) => {
        $n!($a, "063");
    };
    ($n:path,$a:tt,64) => {
        $n!($a, "064");
    };
    ($n:path,$a:tt,65) => {
        $n!($a, "065");
    };
    ($n:path,$a:tt,66) => {
        $n!($a, "066");
    };
    ($n:path,$a:tt,67) => {
        $n!($a, "067");
    };
    ($n:path,$a:tt,68) => {
        $n!($a, "068");
    };
    ($n:path,$a:tt,69) => {
        $n!($a, "069");
    };
    ($n:path,$a:tt,70) => {
        $n!($a, "070");
    };
    ($n:path,$a:tt,71) => {
        $n!($a, "071");
    };
    ($n:path,$a:tt,72) => {
        $n!($a, "072");
    };
    ($n:path,$a:tt,73) => {
        $n!($a, "073");
    };
    ($n:path,$a:tt,74) => {
        $n!($a, "074");
    };
    ($n:path,$a:tt,75) => {
        $n!($a, "075");
    };
    ($n:path,$a:tt,76) => {
        $n!($a, "076");
    };
    ($n:path,$a:tt,77) => {
        $n!($a, "077");
    };
    ($n:path,$a:tt,78) => {
        $n!($a, "078");
    };
    ($n:path,$a:tt,79) => {
        $n!($a, "079");
    };
    ($n:path,$a:tt,80) => {
        $n!($a, "080");
    };
    ($n:path,$a:tt,81) => {
        $n!($a, "081");
    };
    ($n:path,$a:tt,82) => {
        $n!($a, "082");
    };
    ($n:path,$a:tt,83) => {
        $n!($a, "083");
    };
    ($n:path,$a:tt,84) => {
        $n!($a, "084");
    };
    ($n:path,$a:tt,85) => {
        $n!($a, "085");
    };
    ($n:path,$a:tt,86) => {
        $n!($a, "086");
    };
    ($n:path,$a:tt,87) => {
        $n!($a, "087");
    };
    ($n:path,$a:tt,88) => {
        $n!($a, "088");
    };
    ($n:path,$a:tt,89) => {
        $n!($a, "089");
    };
    ($n:path,$a:tt,90) => {
        $n!($a, "090");
    };
    ($n:path,$a:tt,91) => {
        $n!($a, "091");
    };
    ($n:path,$a:tt,92) => {
        $n!($a, "092");
    };
    ($n:path,$a:tt,93) => {
        $n!($a, "093");
    };
    ($n:path,$a:tt,94) => {
        $n!($a, "094");
    };
    ($n:path,$a:tt,95) => {
        $n!($a, "095");
    };
    ($n:path,$a:tt,96) => {
        $n!($a, "096");
    };
    ($n:path,$a:tt,97) => {
        $n!($a, "097");
    };
    ($n:path,$a:tt,98) => {
        $n!($a, "098");
    };
    ($n:path,$a:tt,99) => {
        $n!($a, "099");
    };
    ($n:path,$a:tt,$priority:literal) => {
        $n!($a, $priority);
    };
}
