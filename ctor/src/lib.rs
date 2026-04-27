//! Procedural macro for defining global constructor/destructor functions.
//!
//! This provides module initialization/teardown functions for Rust (like
//! `__attribute__((constructor))` in C/C++) for Linux, OSX, and Windows via
//! the `#[ctor]` and `#[dtor]` macros.
//!
//! This library works and is regularly tested on Linux, OSX and Windows, with both `+crt-static` and `-crt-static`.
//! Other platforms are supported but not tested as part of the automatic builds. This library will also work as expected in both
//! `bin` and `cdylib` outputs, ie: the `ctor` and `dtor` will run at executable or library
//! startup/shutdown respectively.
//!
//! For most platforms, this library currently has a MSRV of **Rust >= 1.60**.

#![no_std]
#![recursion_limit = "256"]

#[cfg(feature = "std")]
extern crate std;

mod macros;

#[doc(hidden)]
#[allow(unused)]
pub mod __support {
    use crate::macros::*;

    #[cfg(feature = "dtor")]
    pub use dtor::declarative::dtor as dtor_parse;

    // Required for proc_macro.
    pub use crate::__ctor_parse as ctor_parse;

    #[macro_export]
    #[doc(hidden)]
    macro_rules! __ctor_parse {
        ( $($input:tt)* ) => {
            $crate::__perform!(
                ($($input)*),
                $crate::__chain[
                    $crate::__parse_item[$crate::__ctor_features],
                    $crate::__ctor_parse_impl,
                ]
            );
        };
    }

    /// Parse a processed `ctor` item. This is intentionally verbose to avoid
    /// excessive nesting of macro calls in user code.
    #[macro_export]
    #[doc(hidden)]
    macro_rules! __ctor_parse_impl {
        // Step 0: Check function shape
        ( @entry next=$next:path[$next_args:tt], input=(
            features = (
                anonymous = $anonymous:tt,
                crate_path = $crate_path:tt,
                link_section = $link_section:tt,
                no_warn_on_missing_unsafe = $no_warn_on_missing_unsafe:tt,
                priority = $priority:tt,
                proc_macro = $proc_macro:tt,
                std = $std:tt,
                used_linker = $used_linker:tt,
            ),
            meta = $meta:tt,
            item = ($vis:vis unsafe $( extern $abi:literal )? fn $name:ident () $( -> () )? {
                $($body:tt)*
            })
        ) ) => {
            $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
                features = (
                    anonymous = $anonymous,
                    link_section = $link_section,
                    no_warn_on_missing_unsafe = $no_warn_on_missing_unsafe,
                    priority = $priority,
                    used_linker = $used_linker,
                ),
                meta = $meta,
                item = ($vis unsafe $( extern $abi )? fn $name () {
                    $($body)*
                })
            ));
        };

        ( @entry next=$next:path[$next_args:tt], input=(
            features = (
                anonymous = $anonymous:tt,
                crate_path = $crate_path:tt,
                link_section = $link_section:tt,
                no_warn_on_missing_unsafe = $no_warn_on_missing_unsafe:tt,
                priority = $priority:tt,
                proc_macro = $proc_macro:tt,
                std = $std:tt,
                used_linker = $used_linker:tt,
            ),
            meta = $meta:tt,
            item = ($vis:vis $( extern $abi:literal )? fn $name:ident () $( -> () )? {
                $($body:tt)*
            })
        ) ) => {
            $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
                features = (
                    anonymous = $anonymous,
                    link_section = $link_section,
                    no_warn_on_missing_unsafe = $no_warn_on_missing_unsafe,
                    priority = $priority,
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
                link_section = $link_section:tt,
                no_warn_on_missing_unsafe = $no_warn_on_missing_unsafe:tt,
                priority = $priority:tt,
                proc_macro = $proc_macro:tt,
                std = $std:tt,
                used_linker = $used_linker:tt,
            ),
            meta = $meta:tt,
            item = ($vis:vis static $ident:ident : $ty:ty = $(unsafe)? { $literal:literal };)
        ) ) => {
            compile_error!("Trivial const expressions are not supported. Remove the #[ctor] and use a regular `static`.");
        };

        ( @entry next=$next:path[$next_args:tt], input=(
            features = (
                anonymous = $anonymous:tt,
                crate_path = $crate_path:tt,
                link_section = $link_section:tt,
                no_warn_on_missing_unsafe = $no_warn_on_missing_unsafe:tt,
                priority = $priority:tt,
                proc_macro = $proc_macro:tt,
                std = $std:tt,
                used_linker = $used_linker:tt,
            ),
            meta = $meta:tt,
            item = ($vis:vis static $ident:ident : $ty:ty = const { $literal:literal };)
        ) ) => {
            compile_error!("Trivial const expressions are not supported. Remove the #[ctor] and use a regular `static`.");
        };

        ( @entry next=$next:path[$next_args:tt], input=(
            features = (
                anonymous = $anonymous:tt,
                crate_path = $crate_path:tt,
                link_section = $link_section:tt,
                no_warn_on_missing_unsafe = $no_warn_on_missing_unsafe:tt,
                priority = $priority:tt,
                proc_macro = $proc_macro:tt,
                std = $std:tt,
                used_linker = $used_linker:tt,
            ),
            meta = $meta:tt,
            item = ($vis:vis static $ident:ident : $ty:ty = $literal:literal;)
        ) ) => {
            compile_error!("Trivial const expressions are not supported. Remove the #[ctor] and use a regular `static`.");
        };

        ( @entry next=$next:path[$next_args:tt], input=(
            features = (
                anonymous = $anonymous:tt,
                crate_path = $crate_path:tt,
                link_section = $link_section:tt,
                no_warn_on_missing_unsafe = $no_warn_on_missing_unsafe:tt,
                priority = $priority:tt,
                proc_macro = $proc_macro:tt,
                std = $std:tt,
                used_linker = $used_linker:tt,
            ),
            meta = $meta:tt,
            item = ($vis:vis static $ident:ident : $ty:ty = unsafe { $($body:tt)* };)
        ) ) => {
            $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
                features = (
                    anonymous = $anonymous,
                    link_section = $link_section,
                    no_warn_on_missing_unsafe = $no_warn_on_missing_unsafe,
                    priority = $priority,
                    used_linker = $used_linker,
                ),
                meta = $meta,
                item = ($vis static $ident : $ty = unsafe { $($body)* };)
            ));
        };

        ( @entry next=$next:path[$next_args:tt], input=(
            features = (
                anonymous = $anonymous:tt,
                crate_path = $crate_path:tt,
                link_section = $link_section:tt,
                no_warn_on_missing_unsafe = $no_warn_on_missing_unsafe:tt,
                priority = $priority:tt,
                proc_macro = $proc_macro:tt,
                std = $std:tt,
                used_linker = $used_linker:tt,
            ),
            meta = $meta:tt,
            item = ($vis:vis static $ident:ident : $ty:ty = { $($body:tt)* };)
        ) ) => {
            $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
                features = (
                    anonymous = $anonymous,
                    link_section = $link_section,
                    no_warn_on_missing_unsafe = $no_warn_on_missing_unsafe,
                    priority = $priority,
                    used_linker = $used_linker,
                ),
                meta = $meta,
                item = ($vis static $ident : $ty = { $($body)* };)
            ));
        };

        ( @entry next=$next:path[$next_args:tt], input=(
            features = (
                anonymous = $anonymous:tt,
                crate_path = $crate_path:tt,
                link_section = $link_section:tt,
                no_warn_on_missing_unsafe = $no_warn_on_missing_unsafe:tt,
                priority = $priority:tt,
                proc_macro = $proc_macro:tt,
                std = $std:tt,
                used_linker = $used_linker:tt,
            ),
            meta = $meta:tt,
            item = ($item:item)
        ) ) => {
            compile_error!("Invalid ctor item. \
                Expected a function with no args, \
                return value, or type parameters or a static variable.\n\
                Valid forms are:\n\
                 - [pub] [unsafe] [extern $abi] fn $name() { ... }\n\
                 - static $name : $ty = [unsafe] { ... };");
        };

        // Step 1: Compute priority
        ( @entry next=$next:path[$next_args:tt], input=(
            features = (
                anonymous = $anonymous:tt,
                link_section = $link_section:tt,
                no_warn_on_missing_unsafe = $no_warn_on_missing_unsafe:tt,
                priority = (),
                used_linker = $used_linker:tt,
            ),
            meta = $meta:tt,
            item = $item:tt
        ) ) => {
            $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
                features = (
                    anonymous = $anonymous,
                    link_section = ($link_section),
                    no_warn_on_missing_unsafe = $no_warn_on_missing_unsafe,
                    used_linker = $used_linker,
                ),
                meta = $meta,
                item = $item
            ));
        };

        ( @entry next=$next:path[$next_args:tt], input=(
            features = (
                anonymous = $anonymous:tt,
                link_section = $link_section:tt,
                no_warn_on_missing_unsafe = $no_warn_on_missing_unsafe:tt,
                priority = $priority:literal,
                used_linker = $used_linker:tt,
            ),
            meta = $meta:tt,
            item = $item:tt
        ) ) => {
            $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
                features = (
                    anonymous = $anonymous,
                    link_section = ($link_section), //(concat!($link_section, ".", $priority)),
                    no_warn_on_missing_unsafe = $no_warn_on_missing_unsafe,
                    used_linker = $used_linker,
                ),
                meta = $meta,
                item = $item
            ));
        };

        // Step 2: Compute no_warn_on_missing_unsafe
        ( @entry next=$next:path[$next_args:tt], input=(
            features = (
                anonymous = $anonymous:tt,
                link_section = $link_section:tt,
                no_warn_on_missing_unsafe = (),
                used_linker = $used_linker:tt,
            ),
            meta = $meta:tt,
            item = $item:tt
        ) ) => {
            $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
                features = (
                    anonymous = $anonymous,
                    link_section = $link_section,
                    used_linker = $used_linker,
                ),
                meta = $meta,
                item = $item
            ));
        };

        ( @entry next=$next:path[$next_args:tt], input=(
            features = (
                anonymous = $anonymous:tt,
                link_section = $link_section:tt,
                no_warn_on_missing_unsafe = no_warn_on_missing_unsafe,
                used_linker = $used_linker:tt,
            ),
            meta = $meta:tt,
            item = $item:tt
        ) ) => {
            $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
                features = (
                    anonymous = $anonymous,
                    link_section = $link_section,
                    used_linker = $used_linker,
                ),
                meta = $meta,
                item = $item
            ));
        };

        // Step 3: Wrap in anonymous const
        ( @entry next=$next:path[$next_args:tt], input=(
            features = (
                anonymous = (),
                link_section = $link_section:tt,
                used_linker = $used_linker:tt,
            ),
            meta = $meta:tt,
            item = $item:tt
        ) ) => {
            $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
                features = (
                    link_section = $link_section,
                    used_linker = $used_linker,
                ),
                meta = $meta,
                item = $item
            ));
        };
        ( @entry next=$next:path[$next_args:tt], input=(
            features = (
                anonymous = anonymous,
                link_section = $link_section:tt,
                used_linker = $used_linker:tt,
            ),
            meta = $meta:tt,
            item = $item:tt
        ) ) => {
            const _: () = {
                $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
                    features = (
                        link_section = $link_section,
                        used_linker = $used_linker,
                    ),
                    meta = $meta,
                    item = $item
                ));
            };
        };

        // Step 4: Compute used_linker
        ( @entry next=$next:path[$next_args:tt], input=(
            features = (
                link_section = $link_section:tt,
                used_linker = (),
            ),
            meta = $meta:tt,
            item = $item:tt
        ) ) => {
            $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
                features = (
                    link_section = $link_section,
                    used_linker_meta = (#[used]),
                ),
                meta = $meta,
                item = $item
            ));
        };

        ( @entry next=$next:path[$next_args:tt], input=(
            features = (
                link_section = $link_section:tt,
                used_linker = used_linker,
            ),
            meta = $meta:tt,
            item = $item:tt
        ) ) => {
            $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
                features = (
                    link_section = $link_section,
                    used_linker_meta = (#[used(linker)]),
                ),
                meta = $meta,
                item = $item
            ));
        };

        // Step 5: Delegate on item type
        ( @entry next=$next:path[$next_args:tt], input=(
            features = (
                link_section = ($($link_section:tt)*),
                used_linker_meta = (#$used_linker_meta:tt),
            ),
            meta = ($($meta:tt)*),
            item = ($vis:vis unsafe $( extern $abi:literal )? fn $name:ident () $( -> () )? {
                $($body:tt)*
            })
        ) ) => {
            $($meta)*
            $vis unsafe $( extern $abi )? fn $name () {
                const _: () = {
                    #[link_section = $($link_section)*]
                    #$used_linker_meta
                    #[allow(non_upper_case_globals)]
                    static __CTOR__PRIVATE__REF__: unsafe extern "C" fn() = {
                        #[allow(non_snake_case)]
                        extern "C" fn __ctor__private__() {
                            unsafe { $name() }
                        }
                        __ctor__private__
                    };
                };
                $($body)*
            }
        };

        ( @entry next=$next:path[$next_args:tt], input=(
            features = (
                link_section = ($($link_section:tt)*),
                used_linker_meta = (#$used_linker_meta:tt),
            ),
            meta = ($($meta:tt)*),
            item = ($vis:vis $( extern $abi:literal )? fn $name:ident () $( -> () )? {
                $($body:tt)*
            })
        ) ) => {
            $($meta)*
            $vis $( extern $abi )? fn $name () {
                const _: () = {
                    #[link_section = $($link_section)*]
                    #$used_linker_meta
                    #[allow(non_upper_case_globals)]
                    static __CTOR__PRIVATE__REF__: unsafe extern "C" fn() = {
                        #[allow(non_snake_case)]
                        extern "C" fn __ctor__private__() {
                            unsafe { $name() }
                        }
                        __ctor__private__
                    };
                };
                $($body)*
            }
        };

        ( @entry next=$next:path[$next_args:tt], input=(
            features = (
                link_section = ($($link_section:tt)*),
                used_linker_meta = (#$used_linker_meta:tt),
            ),
            meta = ($($meta:tt)*),
            item = ($vis:vis static $ident:ident : $ty:ty = $(unsafe)? { $($body:tt)* };)
        ) ) => {
            $($meta)*
            $vis static $ident: $crate::statics::Static<$ty> = {
                fn init() -> $ty {
                    $($body)*
                }
                unsafe { $crate::statics::Static::<$ty>::new(init) }
            };

            const _: () = {
                #[link_section = $($link_section)*]
                #$used_linker_meta
                #[allow(non_upper_case_globals)]
                static __CTOR__PRIVATE__REF__: unsafe extern "C" fn() = {
                    #[allow(non_snake_case)]
                    extern "C" fn __ctor__private__() {
                        unsafe { _ = &*$ident; }
                    }
                    __ctor__private__
                };
            };
        };

    }
}

pub mod statics {
    use core::cell::UnsafeCell;
    use core::mem::MaybeUninit;
    use core::ops::Deref;
    use core::ptr;
    use core::sync::atomic::{AtomicU8, Ordering};

    /// A static variable intended to be initialized before `main`.
    ///
    /// ## Expected usage
    ///
    /// This type is designed for the "startup is single-threaded" model: it is
    /// expected that no other threads access the value until initialization has
    /// completed. After that point, the value is treated as initialized and
    /// immutable.
    ///
    /// ## Concurrency
    ///
    /// If the value is accessed concurrently while it is still being
    /// initialized (for example, from another thread during startup, including
    /// when used from a dynamic library), this is not undefined behavior, but
    /// this implementation does not wait for initialization to complete: it
    /// will panic instead.
    ///
    /// If you need concurrent first access to be handled by blocking/spinning
    /// rather than panicking, use [`std::sync::OnceLock`] or
    /// [`std::sync::LazyLock`] (when `std` is available).
    ///
    /// ## Panics / poisoning
    ///
    /// If the initializer panics, the static becomes permanently poisoned: all
    /// subsequent accesses will panic, and it cannot be reset.
    pub struct Static<T: Sync> {
        storage: UnsafeCell<MaybeUninit<T>>,
        initializer: fn() -> T,
        initialized: AtomicU8,
    }

    const UNINITIALIZED: u8 = 0;
    const INITIALIZING: u8 = 1;
    const FINISHED_INITIALIZING: u8 = 2;
    const INITIALIZED: u8 = 3;
    const DROPPING: u8 = 4;
    const POISONED: u8 = 0x10;
    const DROPPED: u8 = 0xff;

    unsafe impl<T: Sync> Sync for Static<T> {}

    impl<T: Sync> Static<T> {
        /// Create a new ctor-initialized static variable.
        ///
        /// # Safety
        ///
        /// See the documentation for `Static` for more information.
        #[doc(hidden)]
        pub const unsafe fn new(initializer: fn() -> T) -> Self {
            Self {
                storage: UnsafeCell::new(MaybeUninit::uninit()),
                initializer,
                initialized: AtomicU8::new(UNINITIALIZED),
            }
        }

        #[inline(always)]
        unsafe fn get_unchecked(&self) -> &T {
            unsafe {
                (UnsafeCell::raw_get(&self.storage) as *const T)
                    .as_ref()
                    .unwrap_unchecked()
            }
        }
    }

    impl<T: Sync> Deref for Static<T> {
        type Target = T;
        #[inline]
        fn deref(&self) -> &Self::Target {
            struct PanicGuard<'a> {
                initialized: &'a AtomicU8,
            }
            impl<'a> Drop for PanicGuard<'a> {
                fn drop(&mut self) {
                    self.initialized.fetch_or(POISONED, Ordering::AcqRel);
                }
            }

            unsafe {
                match self.initialized.fetch_or(INITIALIZING, Ordering::AcqRel) {
                    UNINITIALIZED => {
                        let panic_guard = PanicGuard {
                            initialized: &self.initialized,
                        };
                        let value = (self.initializer)();
                        core::mem::forget(panic_guard);
                        ptr::write(self.storage.get() as _, value);
                        self.initialized
                            .fetch_or(FINISHED_INITIALIZING, Ordering::AcqRel);
                        self.get_unchecked()
                    }
                    INITIALIZING => {
                        panic!("Recursive or overlapping initialization of static variable");
                    }
                    INITIALIZED => self.get_unchecked(),
                    x if x & POISONED != 0 => panic!("Static variable has been poisoned"),
                    _ => panic!("Invalid state for static variable"),
                }
            }
        }
    }

    impl<T: Sync> Drop for Static<T> {
        fn drop(&mut self) {
            unsafe {
                if INITIALIZED == self.initialized.fetch_or(DROPPING, Ordering::AcqRel) {
                    (UnsafeCell::raw_get(&self.storage) as *mut T).drop_in_place();
                    self.initialized.fetch_or(DROPPED, Ordering::AcqRel);
                }
            }
        }
    }
}

/// Declarative forms of the `#[ctor]` and `#[dtor]` macros.
///
/// The declarative forms wrap and parse a proc_macro-like syntax like so, and
/// are identical in expansion to the undecorated procedural macros. The
/// declarative forms support the same attribute parameters as the procedural
/// macros.
///
/// ```rust
/// # #[cfg(not(miri))] mod test { use ctor::*; use libc_print::*;
/// ctor::declarative::ctor! {
///   #[ctor]
///   fn foo() {
///     libc_println!("Hello, world!");
///   }
/// }
/// # }
///
/// // ... the above is identical to:
///
/// # #[cfg(not(miri))] mod test_2 { use ctor::*; use libc_print::*;
/// #[ctor]
/// fn foo() {
///   libc_println!("Hello, world!");
/// }
/// # }
/// ```
pub mod declarative {
    #[doc(inline)]
    pub use crate::__support::ctor_parse as ctor;
    #[doc(inline)]
    #[cfg(feature = "dtor")]
    pub use crate::__support::dtor_parse as dtor;
}

/// Marks a function or static variable as a library/executable constructor.
/// This uses OS-specific linker sections to call a specific function at load
/// time.
///
/// # Important notes
///
/// Rust does not make any guarantees about stdlib support for life-before or
/// life-after main. This means that the `ctor` crate may not work as expected
/// in some cases, such as when used in an `async` runtime or making use of
/// stdlib services.
///
/// Multiple startup functions/statics are supported, but the invocation order
/// is not guaranteed.
///
/// The `ctor` crate assumes it is available as a direct dependency, If you
/// re-export `ctor` items as part of your crate, you can use the `crate_path`
/// parameter to redirect the macro's output to the correct crate, or use the
/// [`declarative::ctor`] form.
///
/// # Attribute parameters
///
///  - `unsafe`: Removes the requirement to mark the function as `unsafe`
///    (recommended).
///  - `link_section = "section"`: The section to place the constructor in.
///  - `anonymous`: Do not give the constructor a name in the generated code
///    (allows for multiple constructors with the same name). Equivalent to
///    wrapping the constructor in an anonymous const (ie: `const _ = { ...
///    };`).
///  - `priority = N`: The priority of the constructor. Higher-N-priority
///    constructors are run last. `N` must be between 0 and 999 for ordering
///    guarantees (N >= 1000 ordering is platform-defined). Ordering with
///    respect to constructors without a priority is platform-defined.
///  - `crate_path = (Advanced) ::path::to::ctor::crate`: The path to the `ctor`
///    crate containing the support macros. If you re-export `ctor` items as
///    part of your crate, you can use this to redirect the macro's output to
///    the correct crate. Using the [`declarative::ctor`] form is preferred over
///    this parameter.
///  - `used(linker)`: (Advanced) Mark the function as being used in the link
///    phase.
///
/// # Examples
///
/// Print a startup message (using `libc_print` for safety):
///
/// ```rust
/// # #![cfg_attr(feature="used_linker", feature(used_with_arg))]
/// # #[cfg(not(miri))] mod test {
/// # use ctor::ctor;
/// use libc_print::std_name::println;
///
/// #[ctor(unsafe)]
/// fn foo() {
///   // Using libc_print which is safe in `#[ctor]`
///   println!("Hello, world!");
/// }
///
/// # fn main() {
/// println!("main()");
/// # }}
/// ```
///
/// Make changes to `static` variables:
///
/// ```rust
/// # #![cfg_attr(feature="used_linker", feature(used_with_arg))]
/// # mod test {
/// # use ctor::*;
/// # use std::sync::atomic::{AtomicBool, Ordering};
/// static INITED: AtomicBool = AtomicBool::new(false);
///
/// #[ctor(unsafe)]
/// fn set_inited() {
///   INITED.store(true, Ordering::SeqCst);
/// }
/// # }
/// ```
///
/// Initialize a `HashMap` at startup time:
///
/// ```rust
/// # #![cfg_attr(feature="used_linker", feature(used_with_arg))]
/// # mod test {
/// # use std::collections::HashMap;
/// # use ctor::*;
/// #[ctor(unsafe)]
/// pub static STATIC_CTOR: HashMap<u32, String> = {
///   let mut m = HashMap::new();
///   for i in 0..100 {
///     m.insert(i, format!("x*100={}", i*100));
///   }
///   m
/// };
/// # }
/// # pub fn main() {
/// #   assert_eq!(test::STATIC_CTOR.len(), 100);
/// #   assert_eq!(test::STATIC_CTOR[&20], "x*100=2000");
/// # }
/// ```
///
/// # Details
///
/// The `#[ctor]` macro makes use of linker sections to ensure that a function
/// is run at startup time.
///
/// ```rust
/// # #![cfg_attr(feature="used_linker", feature(used_with_arg))]
/// # mod test {
/// # use ctor::*;
/// #[ctor(unsafe)]
/// fn my_init_fn() {
///   /* ... */
/// }
/// # }
/// ```
///
/// The above example translates into the following Rust code (approximately):
///
/// ```rust
/// # fn my_init_fn() {}
/// #[used]
/// #[cfg_attr(target_os = "linux", link_section = ".init_array")]
/// #[cfg_attr(target_vendor = "apple", link_section = "__DATA,__mod_init_func,mod_init_funcs")]
/// #[cfg_attr(target_os = "windows", link_section = ".CRT$XCU")]
/// /* ... other platforms elided ... */
/// static INIT_FN: extern fn() = {
///     extern fn init_fn() { my_init_fn(); };
///     init_fn
/// };
/// ```
///
/// For `static` items, the macro generates a `std::sync::OnceLock` that is
/// initialized at startup time. `#[ctor]` on `static` items requires the
/// default `std` feature.
///
/// ```rust
/// # mod test {
/// # use ctor::*;
/// # use std::collections::HashMap;
/// #[ctor]
/// static FOO: HashMap<u32, String> = unsafe {
///   let mut m = HashMap::new();
///   for i in 0..100 {
///     m.insert(i, format!("x*100={}", i*100));
///   }
///   m
/// };
/// # }
/// ```
///
/// The above example translates into the following Rust code (approximately),
/// which eagerly initializes the `HashMap` inside a `OnceLock` at startup time:
///
/// ```rust
/// # mod test {
/// # use ctor::ctor;
/// # use std::collections::HashMap;
/// static FOO: FooStatic = FooStatic { value: ::std::sync::OnceLock::new() };
/// struct FooStatic {
///   value: ::std::sync::OnceLock<HashMap<u32, String>>,
/// }
///
/// impl ::core::ops::Deref for FooStatic {
///   type Target = HashMap<u32, String>;
///   fn deref(&self) -> &Self::Target {
///     self.value.get_or_init(|| unsafe {
///       let mut m = HashMap::new();
///       for i in 0..100 {
///         m.insert(i, format!("x*100={}", i*100));
///       }
///       m
///     })
///   }
/// }
///
/// #[ctor]
/// unsafe fn init_foo_ctor() {
///   _ = &*FOO;
/// }
/// # }
/// ```
#[doc(inline)]
#[cfg(feature = "proc_macro")]
pub use ctor_proc_macro::ctor;

/// Re-exported `#[dtor]` proc-macro from `dtor` crate.
///
/// See [`::dtor`] for more details.
#[doc(inline)]
#[cfg(all(feature = "dtor", feature = "proc_macro"))]
pub use dtor::__dtor_from_ctor as dtor; // note: this is the dtor proc macro that looks in ctor

__declare_features!(
    ctor: __ctor_features;

    /// Make the ctor function anonymous.
    anonymous {
        attr: [(anonymous) => (anonymous)];
    };
    /// Specify a custom crate path for the `ctor` crate. Used when re-exporting the ctor macro.
    crate_path {
        attr: [(crate_path = $path:pat) => (($path))];
        example: "crate_path = ::path::to::ctor::crate";
    };
    /// Place the destructor function pointer in a custom link section.
    link_section {
        attr: [(link_section = $section:literal) => ($section)];
        example: "link_section = \".ctors\"";
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
            // Windows targets: CRT$XPU
            (all(target_vendor = "pc", any(target_env = "gnu", target_env = "msvc"))) => ".CRT$XCU",
            // ... except GNU
            (all(target_vendor = "pc", not(any(target_env = "gnu", target_env = "msvc")))) => ".ctors",
            _ => (compile_error!("Unsupported target for #[ctor]"))
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
    priority {
        attr: [(priority = $priority_value:literal) => ($priority_value)];
        validate: [(priority = $priority:literal), (priority = early), (priority = late)];
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
__generate_docs!(__ctor_features);
