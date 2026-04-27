#![recursion_limit = "256"]
#![no_std]
#![doc = include_str!("../docs/GENERATED.md")]

#[cfg(feature = "std")]
extern crate std;

mod macros;
mod native;

#[cfg(feature = "export_native")]
pub use native::*;

/// Marks a function as a library/executable destructor. This uses OS-specific
/// linker sections to call a specific function at termination time.
///
/// Multiple shutdown functions are supported, but the invocation order is not
/// guaranteed.
///
/// # Attribute parameters
///
///  - `crate_path = ::path::to::dtor::crate`: The path to the `dtor` crate
///    containing the support macros. If you re-export `dtor` items as part of
///    your crate, you can use this to redirect the macro's output to the
///    correct crate.
///  - `used(linker)`: (Advanced) Mark the function as being used in the link
///    phase.
///  - `link_section = "section"`: The section to place the dtor's code in.
///  - `anonymous`: Do not give the destructor a name in the generated code
///    (allows for multiple destructors with the same name).
///
/// ```rust,ignore
/// # #![cfg_attr(feature="used_linker", feature(used_with_arg))]
/// # use dtor::dtor;
/// # fn main() {}
///
/// #[dtor]
/// fn shutdown() {
///   /* ... */
/// }
/// ```
#[doc(inline)]
#[cfg(feature = "proc_macro")]
pub use dtor_proc_macro::dtor;

#[doc(hidden)]
#[cfg(feature = "proc_macro")]
pub use dtor_proc_macro::__dtor_from_ctor;

/// Declarative forms of the `#[dtor]` macro.
///
/// The declarative forms wrap and parse a proc_macro-like syntax like so, and
/// are identical in expansion to the undecorated procedural macros. The
/// declarative forms support the same attribute parameters as the procedural
/// macros.
///
/// ```rust
/// # #[cfg(any())] mod test { use dtor::*; use libc_print::*;
/// dtor::declarative::dtor! {
///   #[dtor]
///   fn foo() {
///     libc_println!("Goodbye, world!");
///   }
/// }
/// # }
///
/// // ... the above is identical to:
///
/// # #[cfg(any())] mod test_2 { use dtor::*; use libc_print::*;
/// #[dtor]
/// fn foo() {
///   libc_println!("Goodbye, world!");
/// }
/// # }
/// ```
pub mod declarative {
    #[doc(inline)]
    pub use crate::__dtor_parse as dtor;
}

#[doc(hidden)]
#[allow(unused)]
pub mod __support {
    use crate::macros::*;

    // Required for proc_macro.
    pub use crate::__dtor_parse as dtor_parse;

    #[macro_export]
    #[doc(hidden)]
    macro_rules! __dtor_parse {
        ( $($input:tt)* ) => {
            $crate::__perform!(
                ($($input)*),
                $crate::__chain[
                    $crate::__parse_item[$crate::__dtor_features],
                    $crate::__dtor_parse_impl,
                ]
            );
        };
    }

    /// Parse a processed `dtor` item. This is intentionally verbose to avoid
    /// excessive nesting of macro calls in user code.
    #[macro_export]
    #[doc(hidden)]
    macro_rules! __dtor_parse_impl {
        // Step 0: Check function shape

        // unsafe fn
        ( @entry next=$next:path[$next_args:tt], input=(
            features = (
                anonymous = $anonymous:tt,
                crate_path = $crate_path:tt,
                ctor_link_section = $ctor_link_section:tt,
                default_term_method = $default_term_method:tt,
                default_unload_method = $default_unload_method:tt,
                link_section = $link_section:tt,
                method = $method:tt,
                no_warn_on_missing_unsafe = $no_warn_on_missing_unsafe:tt,
                proc_macro = $proc_macro:tt,
                std = $std:tt,
                used_linker = $used_linker:tt,
            ),
            meta = $meta:tt,
            item = ($vis:vis unsafe $( extern $abi:literal )? fn $name:ident () $( -> () )? {
                $($body:tt)*
            })
        ) ) => {
            $crate::__dtor_parse_impl!(@entry next=$next[$next_args], input=(
                features = (
                    anonymous = $anonymous,
                    ctor_link_section = $ctor_link_section,
                    default_term_method = $default_term_method,
                    default_unload_method = $default_unload_method,
                    link_section = $link_section,
                    method = $method,
                    no_warn_on_missing_unsafe = $no_warn_on_missing_unsafe,
                    used_linker = $used_linker,
                ),
                meta = $meta,
                item = ($vis unsafe $( extern $abi )? fn $name () {
                    $($body)*
                })
            ));
        };

        // non-unsafe fn
        ( @entry next=$next:path[$next_args:tt], input=(
            features = (
                anonymous = $anonymous:tt,
                crate_path = $crate_path:tt,
                ctor_link_section = $ctor_link_section:tt,
                default_term_method = $default_term_method:tt,
                default_unload_method = $default_unload_method:tt,
                link_section = $link_section:tt,
                method = $method:tt,
                no_warn_on_missing_unsafe = $no_warn_on_missing_unsafe:tt,
                proc_macro = $proc_macro:tt,
                std = $std:tt,
                used_linker = $used_linker:tt,
            ),
            meta = $meta:tt,
            item = ($vis:vis $( extern $abi:literal )? fn $name:ident () $( -> () )? {
                $($body:tt)*
            })
        ) ) => {
            $crate::__dtor_parse_impl!(@entry next=$next[$next_args], input=(
                features = (
                    anonymous = $anonymous,
                    ctor_link_section = $ctor_link_section,
                    default_term_method = $default_term_method,
                    default_unload_method = $default_unload_method,
                    link_section = $link_section,
                    method = $method,
                    no_warn_on_missing_unsafe = $no_warn_on_missing_unsafe,
                    used_linker = $used_linker,
                ),
                meta = $meta,
                item = ($vis $( extern $abi )? fn $name () {
                    $($body)*
                })
            ));
        };

        ( @entry next=$next:path[$next_args:tt], input=(
            features = (
                anonymous = $anonymous:tt,
                crate_path = $crate_path:tt,
                ctor_link_section = $ctor_link_section:tt,
                default_term_method = $default_term_method:tt,
                default_unload_method = $default_unload_method:tt,
                link_section = $link_section:tt,
                method = $method:tt,
                no_warn_on_missing_unsafe = $no_warn_on_missing_unsafe:tt,
                proc_macro = $proc_macro:tt,
                std = $std:tt,
                used_linker = $used_linker:tt,
            ),
            meta = $meta:tt,
            item = ($item:item)
        ) ) => {
            compile_error!("Invalid dtor function. \
                Expected a function with no args, \
                return value, or type parameters.\n\
                Valid forms are: [pub] [unsafe] [extern $abi] fn $name() { ... }");
        };

        // Step 1: Compute method

        // Delegate term -> default_term_method
        ( @entry next=$next:path[$next_args:tt], input=(
            features = (
                anonymous = $anonymous:tt,
                ctor_link_section = $ctor_link_section:tt,
                default_term_method = $default_term_method:tt,
                default_unload_method = $default_unload_method:tt,
                link_section = $link_section:tt,
                method = term,
                no_warn_on_missing_unsafe = $no_warn_on_missing_unsafe:tt,
                used_linker = $used_linker:tt,
            ),
            meta = $meta:tt,
            item = $item:tt
        ) ) => {
            // method = term
            $crate::__dtor_parse_impl(@entry next=$next[$next_args], input=(
                features = (
                    anonymous = $anonymous,
                    ctor_link_section = $ctor_link_section,
                    link_section = $link_section,
                    method = $default_term_method,
                    no_warn_on_missing_unsafe = $no_warn_on_missing_unsafe,
                    used_linker = $used_linker,
                ),
                meta = $meta,
                item = $item
            ));
        };

        // Delegate unload -> default_unload_method
        ( @entry next=$next:path[$next_args:tt], input=(
            features = (
                anonymous = $anonymous:tt,
                ctor_link_section = $ctor_link_section:tt,
                default_term_method = $default_term_method:tt,
                default_unload_method = $default_unload_method:tt,
                link_section = $link_section:tt,
                method = unload,
                no_warn_on_missing_unsafe = $no_warn_on_missing_unsafe:tt,
                used_linker = $used_linker:tt,
            ),
            meta = $meta:tt,
            item = $item:tt
        ) ) => {
            // method = unload
            $crate::__dtor_parse_impl!(@entry next=$next[$next_args], input=(
                features = (
                    anonymous = $anonymous,
                    ctor_link_section = $ctor_link_section,
                    link_section = $link_section,
                    method = $default_unload_method,
                    no_warn_on_missing_unsafe = $no_warn_on_missing_unsafe,
                    used_linker = $used_linker,
                ),
                meta = $meta,
                item = $item
            ));
        };

        // Other methods, pass through
        ( @entry next=$next:path[$next_args:tt], input=(
            features = (
                anonymous = $anonymous:tt,
                ctor_link_section = $ctor_link_section:tt,
                default_term_method = $default_term_method:tt,
                default_unload_method = $default_unload_method:tt,
                link_section = $link_section:tt,
                method = $method:tt,
                no_warn_on_missing_unsafe = $no_warn_on_missing_unsafe:tt,
                used_linker = $used_linker:tt,
            ),
            meta = $meta:tt,
            item = $item:tt
        ) ) => {
            // method = other
            $crate::__dtor_parse_impl!(@entry next=$next[$next_args], input=(
                features = (
                    anonymous = $anonymous,
                    ctor_link_section = $ctor_link_section,
                    link_section = $link_section,
                    method = $method,
                    no_warn_on_missing_unsafe = $no_warn_on_missing_unsafe,
                    used_linker = $used_linker,
                ),
                meta = $meta,
                item = $item
            ));
        };

        // Step 2: warn on missing unsafe

        // If no_warn_on_missing_unsafe, ignore
        ( @entry next=$next:path[$next_args:tt], input=(
            features = (
                anonymous = $anonymous:tt,
                ctor_link_section = $ctor_link_section:tt,
                link_section = $link_section:tt,
                method = $method:tt,
                no_warn_on_missing_unsafe = no_warn_on_missing_unsafe,
                used_linker = $used_linker:tt,
            ),
            meta = $meta:tt,
            item = $item:tt
        ) ) => {
            $crate::__dtor_parse_impl!(@entry next=$next[$next_args], input=(
                features = (
                    anonymous = $anonymous,
                    ctor_link_section = $ctor_link_section,
                    link_section = $link_section,
                    method = $method,
                    used_linker = $used_linker,
                ),
                meta = $meta,
                item = $item
            ));
        };

        // If unsafe, pass through
        ( @entry next=$next:path[$next_args:tt], input=(
            features = (
                anonymous = $anonymous:tt,
                ctor_link_section = $ctor_link_section:tt,
                link_section = $link_section:tt,
                method = $method:tt,
                no_warn_on_missing_unsafe = (),
                used_linker = $used_linker:tt,
            ),
            meta = $meta:tt,
            item = ($vis:vis unsafe $($rest:tt)*)
        ) ) => {
            $crate::__dtor_parse_impl!(@entry next=$next[$next_args], input=(
                features = (
                    anonymous = $anonymous,
                    ctor_link_section = $ctor_link_section,
                    link_section = $link_section,
                    method = $method,
                    used_linker = $used_linker,
                ),
                meta = $meta,
                item = ($vis unsafe $($rest)*)
            ));
        };

        // If no unsafe and no_warn_on_missing_unsafe is missing, warn
        ( @entry next=$next:path[$next_args:tt], input=(
            features = (
                anonymous = $anonymous:tt,
                ctor_link_section = $ctor_link_section:tt,
                link_section = $link_section:tt,
                method = $method:tt,
                no_warn_on_missing_unsafe = (),
                used_linker = $used_linker:tt,
            ),
            meta = $meta:tt,
            item = $item:tt
        ) ) => {
            const _: () = {
                #[deprecated="dtor deprecation note:\n\n\
                Use of #[dtor] without `#[dtor(unsafe)]` or `unsafe fn` is deprecated. As code execution\n\
                before main is unsupported by most Rust runtime functions, these functions must be marked\n\
                `unsafe`."]
                const fn dtor_without_unsafe_is_deprecated() {}
                #[allow(unused)]
                static UNSAFE_WARNING: () = {
                    dtor_without_unsafe_is_deprecated()
                };
            };

            $crate::__dtor_parse_impl!(@entry next=$next[$next_args], input=(
                features = (
                    anonymous = $anonymous,
                    ctor_link_section = $ctor_link_section,
                    link_section = $link_section,
                    method = $method,
                    used_linker = $used_linker,
                ),
                meta = $meta,
                item = $item
            ));
        };

        // Step 3: Wrap anonymous if needed
        ( @entry next=$next:path[$next_args:tt], input=(
            features = (
                anonymous = anonymous,
                ctor_link_section = $ctor_link_section:tt,
                link_section = $link_section:tt,
                method = $method:tt,
                used_linker = $used_linker:tt,
            ),
            meta = $meta:tt,
            item = $item:tt
        ) ) => {
            // Anonymous function
            const _: () = {
                $crate::__dtor_parse_impl!(@entry next=$next[$next_args], input=(
                    features = (
                        ctor_link_section = $ctor_link_section,
                        link_section = $link_section,
                        method = $method,
                        used_linker = $used_linker,
                    ),
                    meta = $meta,
                    item = $item
                ));
            };
        };

        ( @entry next=$next:path[$next_args:tt], input=(
            features = (
                anonymous = (),
                ctor_link_section = $ctor_link_section:tt,
                link_section = $link_section:tt,
                method = $method:tt,
                used_linker = $used_linker:tt,
            ),
            meta = $meta:tt,
            item = $item:tt
        ) ) => {
            $crate::__dtor_parse_impl!(@entry next=$next[$next_args], input=(
                features = (
                    ctor_link_section = $ctor_link_section,
                    link_section = $link_section,
                    method = $method,
                    used_linker = $used_linker,
                ),
                meta = $meta,
                item = $item
            ));
        };

        // Step 4: Compute used_linker
        ( @entry next=$next:path[$next_args:tt], input=(
            features = (
                ctor_link_section = $ctor_link_section:tt,
                link_section = $link_section:tt,
                method = $method:tt,
                used_linker = (),
            ),
            meta = $meta:tt,
            item = $item:tt
        ) ) => {
            $crate::__dtor_parse_impl!(@entry next=$next[$next_args], input=(
                features = (
                    ctor_link_section = $ctor_link_section,
                    link_section = $link_section,
                    method = $method,
                    used_linker_meta = (#[used]),
                ),
                meta = $meta,
                item = $item
            ));
        };

        ( @entry next=$next:path[$next_args:tt], input=(
            features = (
                ctor_link_section = $ctor_link_section:tt,
                link_section = $link_section:tt,
                method = $method:tt,
                used_linker = used_linker,
            ),
            meta = $meta:tt,
            item = $item:tt
        ) ) => {
            $crate::__dtor_parse_impl!(@entry next=$next[$next_args], input=(
                features = (
                    ctor_link_section = $ctor_link_section,
                    link_section = $link_section,
                    method = $method,
                    used_linker_meta = (#[used(linker)]),
                ),
                meta = $meta,
                item = $item
            ));
        };

        // Step 5: Delegate on method (at_module_exit, at_binary_exit, link_section)
        ( @entry next=$next:path[$next_args:tt], input=(
            features = (
                ctor_link_section = $ctor_link_section:tt,
                link_section = $link_section:tt,
                method = $method:tt,
                used_linker_meta = (#$used_linker_meta:tt),
            ),
            meta = ($($meta:tt)*),
            item = ($vis:vis $( extern $abi:literal )? fn $name:ident $args:tt $( -> () )? {
                $($body:tt)*
            })
        ) ) => {
            $($meta)*
            $vis $( extern $abi )? fn $name $args {
                $crate::__dtor_parse_impl!(@entry next=$next[$next_args], input=(
                    features = (
                        ctor_link_section = $ctor_link_section,
                        link_section = $link_section,
                        method = $method,
                        used_linker_meta = (#$used_linker_meta),
                    ),
                    item = $name
                ));

                $($body)*
            }
        };

        ( @entry next=$next:path[$next_args:tt], input=(
            features = (
                ctor_link_section = $ctor_link_section:tt,
                link_section = $link_section:tt,
                method = $method:tt,
                used_linker_meta = (#$used_linker_meta:tt),
            ),
            meta = ($($meta:tt)*),
            item = ($vis:vis unsafe $( extern $abi:literal )? fn $name:ident $args:tt $( -> () )? {
                $($body:tt)*
            })
        ) ) => {
            $($meta)*
            $vis unsafe $( extern $abi )? fn $name $args {
                $crate::__dtor_parse_impl!(@entry next=$next[$next_args], input=(
                    features = (
                        ctor_link_section = $ctor_link_section,
                        link_section = $link_section,
                        method = $method,
                        used_linker_meta = (#$used_linker_meta),
                    ),
                    item = $name
                ));

                $($body)*
            }
        };

        ( @entry next=$next:path[$next_args:tt], input=(
            features = (
                ctor_link_section = $ctor_link_section:tt,
                link_section = $link_section:tt,
                method = link_section,
                used_linker_meta = (#$used_linker_meta:tt),
            ),
            item = $name:ident
        ) ) => {
            const _: () = {
                #[link_section = $link_section]
                #$used_linker_meta
                #[allow(non_upper_case_globals)]
                static __DTOR__PRIVATE__REF__: extern "C" fn() = {
                    #[allow(non_snake_case)]
                    extern "C" fn __dtor__private__() {
                        unsafe { $name() }
                    }
                    __dtor__private__
                };
            };
        };

        ( @entry next=$next:path[$next_args:tt], input=(
            features = (
                ctor_link_section = $ctor_link_section:tt,
                link_section = $link_section:tt,
                method = at_module_exit,
                used_linker_meta = (#$used_linker_meta:tt),
            ),
            item = $name:ident
        ) ) => {
            const _: () = {
                #[link_section = $ctor_link_section]
                #$used_linker_meta
                #[allow(non_upper_case_globals)]
                static __CTOR__PRIVATE__REF__: unsafe extern "C" fn() = {
                    #[allow(non_snake_case)]
                    unsafe extern "C" fn __ctor__private__() {
                        $crate::__support::at_module_exit(__dtor__private__);
                    }
                    #[allow(non_snake_case)]
                    extern "C" fn __dtor__private__() {
                        unsafe { $name() }
                    }
                    __ctor__private__
                };
            };
        };

        ( @entry next=$next:path[$next_args:tt], input=(
            features = (
                ctor_link_section = $ctor_link_section:tt,
                link_section = $link_section:tt,
                method = at_binary_exit,
                used_linker_meta = (#$used_linker_meta:tt),
            ),
            item = $name:ident
        ) ) => {
            const _: () = {
                #[link_section = $ctor_link_section]
                #$used_linker_meta
                #[allow(non_upper_case_globals)]
                static __CTOR__PRIVATE__REF__: unsafe extern "C" fn() = {
                    #[allow(non_snake_case)]
                    unsafe extern "C" fn __ctor__private__() {
                        $crate::__support::at_binary_exit(__dtor__private__);
                    }
                    #[allow(non_snake_case)]
                    extern "C" fn __dtor__private__() {
                        unsafe { $name() }
                    }
                    __ctor__private__
                };
            };
        };
    }

    pub use crate::native::*;
}

__declare_features!(
    dtor: __dtor_features;

    /// Make the ctor function anonymous.
    anonymous {
        attr: [(anonymous) => (anonymous)];
    };
    /// Specify a custom crate path for the `dtor` crate. Used when re-exporting the dtor macro.
    crate_path {
        attr: [(crate_path = $path:pat) => (($path))];
        example: "crate_path = ::path::to::dtor::crate";
    };
    /// Place the initialization function pointer in a custom link section.
    ctor_link_section {
        attr: [(ctor(link_section = $ctor_link_section_name:literal)) => ($ctor_link_section_name)];
        example: "ctor(link_section = \".ctors\")";
        default {
            // This is no longer supported by Apple
            (target_vendor = "apple") => "__DATA,__mod_init_func,mod_init_funcs",
            // Most LLVM/GCC targets can use .fini_array
            (any(
                target_os = "linux",
                target_os = "android",
                target_os = "freebsd",
                target_os = "netbsd",
                target_os = "openbsd",
                target_os = "dragonfly",
                target_os = "illumos",
                target_os = "haiku",
                target_family = "wasm"
            )) => ".init_array",
            // xtensa targets: .dtors
            (target_arch = "xtensa") => ".ctors",
            // Windows targets: .CRT$XPU (requires static CRT)
            (all(target_vendor = "pc", any(target_env = "gnu", target_env = "msvc"))) => ".CRT$XPU",
            // ... except GNU
            (all(target_vendor = "pc", not(any(target_env = "gnu", target_env = "msvc")))) => ".ctors",
            _ => (compile_error!("Unsupported target for #[ctor]"))
        }
    };
    /// The default method used for running a `dtor` on termination. This is
    /// generally not recommended as code may be unloaded before the dtor is
    /// called.
    ///
    /// This is only used if the specified `dtor` method is `term`.
    ///
    /// All platforms use `at_binary_exit` except Windows, which uses
    /// `at_module_exit`.
    default_term_method {
        default {
            (target_vendor = "pc") => at_module_exit,
            _ => at_binary_exit,
        }
    };
    /// The default method used for running a `dtor` on module unload.
    ///
    /// This is only used if the `method` attribute is not specified, or if the
    /// method is `unload`.
    default_unload_method {
        default {
            _ => at_module_exit,
        }
    };
    /// Place the destructor function pointer in a custom link section.
    link_section {
        attr: [(link_section = $section:literal) => ($section)];
        example: "link_section = \".dtors\"";
        default {
            // This is no longer supported by Apple
            (target_vendor = "apple") => "__DATA,__mod_term_func,mod_term_funcs",
            // Most LLVM/GCC targets can use .fini_array
            (any(
                target_os = "linux",
                target_os = "android",
                target_os = "freebsd",
                target_os = "netbsd",
                target_os = "openbsd",
                target_os = "dragonfly",
                target_os = "illumos",
                target_os = "haiku",
                target_family = "wasm"
            )) => ".fini_array",
            // xtensa targets: .dtors
            (target_arch = "xtensa") => ".dtors",
            // Windows targets: CRT$XPU
            (all(target_vendor = "pc", any(target_env = "gnu", target_env = "msvc"))) => ".CRT$XTU",
            // ... except GNU
            (all(target_vendor = "pc", not(any(target_env = "gnu", target_env = "msvc")))) => ".dtors",
            _ => (compile_error!("Unsupported target for #[dtor]"))
        }
    };
    /// Specify the dtor method.
    ///
    ///  - `term`: Run the dtor on binary termination. Not recommended as code
    ///    may be unloaded before the dtor is called.
    ///  - `unload`: Run the dtor on module unload (library or binary).
    ///  - `at_module_exit`: Run the dtor using `__cxa_atexit`.
    ///  - `at_binary_exit`: Run the dtor using `atexit` (unsupported on Windows
    ///    platforms).
    ///  - `link_section`: Run the dtor using a custom link section (unsupported
    ///    on Apple platforms).
    method {
        attr: [(method = $method_id:ident) => ($method_id)];
        example: "method = term|unload|at_module_exit|at_binary_exit|link_section";
        validate: [(method = term), (method = unload), (method = at_module_exit), (method = at_binary_exit), (method = link_section)];
        default {
            (target_vendor = "apple") => at_module_exit,
            (target_vendor = "pc") => at_module_exit,
            _ => link_section,
        }
    };
    no_warn_on_missing_unsafe {
        /// crate
        /// Do not warn when a ctor or dtor is missing the `unsafe` keyword.
        feature: "no_warn_on_missing_unsafe";
        /// attr
        /// Marks a ctor/dtor as unsafe.
        attr: [(unsafe) => (no_warn_on_missing_unsafe)];
    };
    /// Enable support for the proc-macro `#[dtor]` attribute. The declarative
    /// form (`dtor!(...)`) is always available. It is recommended that crates
    /// re-exporting the `dtor` macro disable this feature and only use the
    /// declarative form.
    proc_macro {
        feature: "proc_macro";
    };
    /// Enable support for the standard library.
    std {
        feature: "std";
    };
    used_linker {
        /// crate
        /// Applies `used(linker)` to all `dtor`-generated functions. Requires nightly and `feature(used_with_arg)`.
        feature: "used_linker";
        /// attr
        /// Mark generated functions for this `dtor` as `used(linker)`. Requires nightly and `feature(used_with_arg)`.
        attr: [(used(linker)) => (used_linker)];
    };
);

#[cfg(doc)]
__generate_docs!(__dtor_features);
