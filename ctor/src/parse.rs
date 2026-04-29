//! Parser for the `#[ctor]` macro.

#[macro_export]
#[doc(hidden)]
macro_rules! __ctor_parse {
    ( $($input:tt)* ) => {
        $crate::__perform!(
            ($($input)*),
            $crate::__chain[
                $crate::__parse_item[$crate::__ctor_features],
                $crate::__extract_unsafe,
                $crate::__ctor_parse_impl,
            ]
        );
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __ctor_parse_internal {
    ( $features:path, $($input:tt)* ) => {
        $crate::__perform!(
            ($($input)*),
            $crate::__chain[
                $crate::__parse_item[$features],
                $crate::__extract_unsafe,
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
    // Step 1: Feature check
    ( @entry next=$next:path[$next_args:tt], input=(
        features = (
            anonymous = $anonymous:tt,
            crate_path = $crate_path:tt,
            export_name_prefix = $export_name_prefix:tt,
            link_section = $link_section:tt,
            no_warn_on_missing_unsafe = $no_warn_on_missing_unsafe:tt,
            priority = $priority:tt,
            priority_enabled = $priority_enabled:tt,
            proc_macro = $proc_macro:tt,
            std = $std:tt,
            used_linker = $used_linker:tt,
        ),
        meta = $meta:tt,
        unsafe = $unsafe:tt,
        item = $item:tt
    )) => {
        $crate::__ctor_parse_impl!(@checkfail priority=$priority priority_enabled=priority_enabled);

        $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
            features = (
                anonymous = $anonymous,
                export_name_prefix = $export_name_prefix,
                link_section = $link_section,
                no_warn_on_missing_unsafe = $no_warn_on_missing_unsafe,
                priority = $priority,
                used_linker = $used_linker,
            ),
            meta = $meta,
            unsafe = $unsafe,
            item = $item
        ));
    };

    ( @checkfail priority=() priority_enabled=$any:tt ) => {};
    ( @checkfail priority=$any:tt priority_enabled=priority_enabled ) => {};
    ( @checkfail priority=$any1:tt priority_enabled=$any2:tt ) => {
        compile_error!("The priority feature is not enabled, so `priority = N` is not supported.");
    };

    ( @checkfail $($rest:tt)* ) => {};

    // Step 2: Check function shape
    ( @entry next=$next:path[$next_args:tt], input=(
        features = (
            anonymous = $anonymous:tt,
            export_name_prefix = $export_name_prefix:tt,
            link_section = $link_section:tt,
            no_warn_on_missing_unsafe = $no_warn_on_missing_unsafe:tt,
            priority = $priority:tt,
            used_linker = $used_linker:tt,
        ),
        meta = $meta:tt,
        unsafe = ($($unsafe:tt)?),
        item = ($vis:vis $(unsafe)? $( extern $abi:literal )? fn $name:ident () $( -> () )? {
            $($body:tt)*
        })
    ) ) => {
        $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
            features = (
                anonymous = $anonymous,
                link_name = $name,
                export_name_prefix = $export_name_prefix,
                link_section = $link_section,
                no_warn_on_missing_unsafe = $no_warn_on_missing_unsafe,
                priority = $priority,
                used_linker = $used_linker,
            ),
            meta = $meta,
            unsafe = ($($unsafe)?),
            item = ($vis $($unsafe)? $( extern $abi )? fn $name () {
                $($body)*
            })
        ));
    };

    ( @entry next=$next:path[$next_args:tt], input=(
        features = (
            anonymous = $anonymous:tt,
            export_name_prefix = $export_name_prefix:tt,
            link_section = $link_section:tt,
            no_warn_on_missing_unsafe = $no_warn_on_missing_unsafe:tt,
            priority = $priority:tt,
            used_linker = $used_linker:tt,
        ),
        meta = $meta:tt,
        unsafe = $unsafe:tt,
        item = ($vis:vis static $ident:ident : $ty:ty = $(unsafe)? { $literal:literal };)
    ) ) => {
        compile_error!("Trivial const expressions are not supported. Remove the #[ctor] and use a regular `static`.");
    };

    ( @entry next=$next:path[$next_args:tt], input=(
        features = (
            anonymous = $anonymous:tt,
            export_name_prefix = $export_name_prefix:tt,
            link_section = $link_section:tt,
            no_warn_on_missing_unsafe = $no_warn_on_missing_unsafe:tt,
            priority = $priority:tt,
            used_linker = $used_linker:tt,
        ),
        meta = $meta:tt,
        unsafe = $unsafe:tt,
        item = ($vis:vis static $ident:ident : $ty:ty = $(unsafe)? const $body:block;)
    ) ) => {
        compile_error!("Trivial const expressions are not supported. Remove the #[ctor] and use a regular `static`.");
    };

    ( @entry next=$next:path[$next_args:tt], input=(
        features = (
            anonymous = $anonymous:tt,
            export_name_prefix = $export_name_prefix:tt,
            link_section = $link_section:tt,
            no_warn_on_missing_unsafe = $no_warn_on_missing_unsafe:tt,
            priority = $priority:tt,
            used_linker = $used_linker:tt,
        ),
        meta = $meta:tt,
        unsafe = $unsafe:tt,
        item = ($vis:vis static $ident:ident : $ty:ty = $(unsafe)? $literal:literal;)
    ) ) => {
        compile_error!("Trivial const expressions are not supported. Remove the #[ctor] and use a regular `static`.");
    };

    ( @entry next=$next:path[$next_args:tt], input=(
        features = (
            anonymous = $anonymous:tt,
            export_name_prefix = $export_name_prefix:tt,
            link_section = $link_section:tt,
            no_warn_on_missing_unsafe = $no_warn_on_missing_unsafe:tt,
            priority = $priority:tt,
            used_linker = $used_linker:tt,
        ),
        meta = $meta:tt,
        unsafe = ($($unsafe:tt)?),
        item = ($vis:vis static $ident:ident : $ty:ty = $(unsafe)? { $($body:tt)* };)
    ) ) => {
        $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
            features = (
                anonymous = $anonymous,
                link_name = $ident,
                export_name_prefix = $export_name_prefix,
                link_section = $link_section,
                no_warn_on_missing_unsafe = $no_warn_on_missing_unsafe,
                priority = $priority,
                used_linker = $used_linker,
            ),
            meta = $meta,
            unsafe = ($($unsafe)?),
            item = ($vis static $ident : $ty = $($unsafe)? { $($body)* };)
        ));
    };

    ( @entry next=$next:path[$next_args:tt], input=(
        features = (
            anonymous = $anonymous:tt,
            export_name_prefix = $export_name_prefix:tt,
            link_section = $link_section:tt,
            no_warn_on_missing_unsafe = $no_warn_on_missing_unsafe:tt,
            priority = $priority:tt,
            used_linker = $used_linker:tt,
        ),
        meta = $meta:tt,
        unsafe = $unsafe:tt,
        item = ($item:item)
    ) ) => {
        compile_error!("Invalid ctor item. \
            Expected a function with no args, \
            return value, or type parameters or a static variable.\n\
            Valid forms are:\n\
             - [pub] [unsafe] [extern $abi] fn $name() { ... }\n\
             - static $name : $ty = [unsafe] { ... };");
    };

    // Step 3: Compute no_warn_on_missing_unsafe

    // warn iff no_warn_on_missing_unsafe is not present AND unsafe is not present
    ( @entry next=$next:path[$next_args:tt], input=(
        features = (
            anonymous = $anonymous:tt,
            link_name = $link_name:tt,
            export_name_prefix = $export_name_prefix:tt,
            link_section = $link_section:tt,
            no_warn_on_missing_unsafe = (),
            priority = $priority:tt,
            used_linker = $used_linker:tt,
        ),
        meta = $meta:tt,
        unsafe = (),
        item = $item:tt
    ) ) => {
        const _: () = {
            #[deprecated="ctor deprecation note:\n\n\
            Use of #[ctor] without `#[ctor(unsafe)]` or `unsafe fn` is deprecated. As code execution\n\
            before main is unsupported by most Rust runtime functions, these functions must be marked\n\
            `unsafe`."]
            const fn ctor_without_unsafe_is_deprecated() {}
            #[allow(unused)]
            static UNSAFE_WARNING: () = {
                ctor_without_unsafe_is_deprecated()
            };
        };

        $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
            features = (
                anonymous = $anonymous,
                link_name = $link_name,
                export_name_prefix = $export_name_prefix,
                link_section = $link_section,
                priority = $priority,
                used_linker = $used_linker,
            ),
            meta = $meta,
            unsafe = (),
            item = $item
        ));
    };

    ( @entry next=$next:path[$next_args:tt], input=(
        features = (
            anonymous = $anonymous:tt,
            link_name = $link_name:tt,
            export_name_prefix = $export_name_prefix:tt,
            link_section = $link_section:tt,
            no_warn_on_missing_unsafe = $no_warn_on_missing_unsafe:tt,
            priority = $priority:tt,
            used_linker = $used_linker:tt,
        ),
        meta = $meta:tt,
        unsafe = $unsafe:tt,
        item = $item:tt
    ) ) => {
        $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
            features = (
                anonymous = $anonymous,
                link_name = $link_name,
                export_name_prefix = $export_name_prefix,
                link_section = $link_section,
                priority = $priority,
                used_linker = $used_linker,
            ),
            meta = $meta,
            unsafe = $unsafe,
            item = $item
        ));
    };

    // Step 4: Wrap in anonymous const
    ( @entry next=$next:path[$next_args:tt], input=(
        features = (
            anonymous = (),
            link_name = $link_name:tt,
            export_name_prefix = $export_name_prefix:tt,
            link_section = $link_section:tt,
            priority = $priority:tt,
            used_linker = $used_linker:tt,
        ),
        meta = $meta:tt,
        unsafe = $unsafe:tt,
        item = $item:tt
    ) ) => {
        $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
            features = (
                link_name = $link_name,
                export_name_prefix = $export_name_prefix,
                link_section = $link_section,
                priority = $priority,
                used_linker = $used_linker,
            ),
            meta = $meta,
            unsafe = $unsafe,
            item = $item
        ));
    };
    ( @entry next=$next:path[$next_args:tt], input=(
        features = (
            anonymous = anonymous,
            link_name = $link_name:tt,
            export_name_prefix = $export_name_prefix:tt,
            link_section = $link_section:tt,
            priority = $priority:tt,
            used_linker = $used_linker:tt,
        ),
        meta = $meta:tt,
        unsafe = $unsafe:tt,
        item = $item:tt
    ) ) => {
        const _: () = {
            $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
                features = (
                    link_name = $link_name,
                    export_name_prefix = $export_name_prefix,
                    link_section = $link_section,
                    priority = $priority,
                    used_linker = $used_linker,
                ),
                meta = $meta,
                unsafe = $unsafe,
                item = $item
            ));
        };
    };

    // Step 5: Compute used_linker
    ( @entry next=$next:path[$next_args:tt], input=(
        features = (
            link_name = $link_name:tt,
            export_name_prefix = $export_name_prefix:tt,
            link_section = $link_section:tt,
            priority = $priority:tt,
            used_linker = (),
        ),
        meta = $meta:tt,
        unsafe = $unsafe:tt,
        item = $item:tt
    ) ) => {
        $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
            features = (
                link_name = $link_name,
                export_name_prefix = $export_name_prefix,
                link_section = $link_section,
                priority = $priority,
                used_linker_meta = (#[used]),
            ),
            meta = $meta,
            unsafe = $unsafe,
            item = $item
        ));
    };

    ( @entry next=$next:path[$next_args:tt], input=(
        features = (
            link_name = $link_name:tt,
            export_name_prefix = $export_name_prefix:tt,
            link_section = $link_section:tt,
            priority = $priority:tt,
            used_linker = used_linker,
        ),
        meta = $meta:tt,
        unsafe = $unsafe:tt,
        item = $item:tt
    ) ) => {
        $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
            features = (
                link_name = $link_name,
                export_name_prefix = $export_name_prefix,
                link_section = $link_section,
                priority = $priority,
                used_linker_meta = (#[used(linker)]),
            ),
            meta = $meta,
            unsafe = $unsafe,
            item = $item
        ));
    };

    // Step 6: Compute export name suffix

    // No prefix, no computation
    ( @entry next=$next:path[$next_args:tt], input=(
        features = (
            link_name = $link_name:tt,
            export_name_prefix = (),
            link_section = $link_section:tt,
            priority = $priority:tt,
            used_linker_meta = $used_linker_meta:tt,
        ),
        meta = $meta:tt,
        unsafe = $unsafe:tt,
        item = $item:tt
    ) ) => {
        $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
            features = (
                export_name = (),
                link_section = $link_section,
                priority = $priority,
                used_linker_meta = $used_linker_meta,
            ),
            meta = $meta,
            unsafe = $unsafe,
            item = $item
        ));
    };

    ( @entry next=$next:path[$next_args:tt], input=(
        features = (
            link_name = $link_name:tt,
            export_name_prefix = $export_name_prefix:tt,
            link_section = $link_section:tt,
            priority = $priority:tt,
            used_linker_meta = $used_linker_meta:tt,
        ),
        meta = $meta:tt,
        unsafe = $unsafe:tt,
        item = $item:tt
    ) ) => {
        $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
            features = (
                export_name = ($export_name_prefix, (concat!(env!("CARGO_PKG_NAME"), "_",
                    ::core::module_path!(), "_",
                    stringify!($link_name),
                    "_L", line!(), "C", column!()))),
                link_section = $link_section,
                priority = $priority,
                used_linker_meta = $used_linker_meta,
            ),
            meta = $meta,
            unsafe = $unsafe,
            item = $item
        ));
    };

    // Step 7: Compute priority

    // No priority, treat as early
    ( @entry next=$next:path[$next_args:tt], input=(
        features = (
            export_name = $export_name:tt,
            link_section = $link_section:tt,
            priority = (),
            used_linker_meta = $used_linker_meta:tt,
        ),
        meta = $meta:tt,
        unsafe = $unsafe:tt,
        item = $item:tt
    ) ) => {
        $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
            features = (
                export_name = $export_name,
                link_section = $link_section,
                priority = early,
                used_linker_meta = $used_linker_meta,
            ),
            meta = $meta,
            unsafe = $unsafe,
            item = $item
        ));
    };

    // naked - no processing
    ( @entry next=$next:path[$next_args:tt], input=(
        features = (
            export_name = (),
            link_section = $link_section:tt,
            priority = naked,
            used_linker_meta = $used_linker_meta:tt,
        ),
        meta = $meta:tt,
        unsafe = $unsafe:tt,
        item = $item:tt
    ) ) => {
        $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
            link_args = (
                export_name = (),
                link_section = ($link_section),
                used = $used_linker_meta,
            ),
            meta = $meta,
            unsafe = $unsafe,
            item = $item
        ));
    };

    // naked with export name: use 0 priority (AIX requires this, could probably be improved)
    ( @entry next=$next:path[$next_args:tt], input=(
        features = (
            export_name = ($($prefix:tt)*, $($suffix:tt)*),
            link_section = $link_section:tt,
            priority = naked,
            used_linker_meta = $used_linker_meta:tt,
        ),
        meta = $meta:tt,
        unsafe = $unsafe:tt,
        item = $item:tt
    ) ) => {
        $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
            link_args = (
                export_name = (concat!($($prefix)*, "0", $($suffix)*)),
                link_section = ($link_section),
                used = $used_linker_meta,
            ),
            meta = $meta,
            unsafe = $unsafe,
            item = $item
        ));
    };

    ( @entry next=$next:path[$next_args:tt], input=(
        features = (
            export_name = $export_name:tt,
            link_section = $link_section:tt,
            priority = early,
            used_linker_meta = $used_linker_meta:tt,
        ),
        meta = $meta:tt,
        unsafe = $unsafe:tt,
        item = $item:tt
    ) ) => {
        #[cfg(target_vendor = "apple")]
        $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
            link_args = (
                export_name = $export_name,
                priority = early,
                used = $used_linker_meta,
            ),
            meta = $meta,
            unsafe = $unsafe,
            item = $item
        ));

        // Treat early as naked for all other platforms
        $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
            features = (
                export_name = $export_name,
                link_section = $link_section,
                priority = naked,
                used_linker_meta = $used_linker_meta,
            ),
            meta = $meta,
            unsafe = $unsafe,
            item = $item
        ));
    };

    ( @entry next=$next:path[$next_args:tt], input=(
        features = (
            export_name = $export_name:tt,
            link_section = $link_section:tt,
            priority = late,
            used_linker_meta = $used_linker_meta:tt,
        ),
        meta = $meta:tt,
        unsafe = $unsafe:tt,
        item = $item:tt
    ) ) => {
        #[cfg(target_vendor = "apple")]
        $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
            link_args = (
                export_name = $export_name,
                priority = late,
                used = $used_linker_meta,
            ),
            meta = $meta,
            unsafe = $unsafe,
            item = $item
        ));

        // Treat late as 65535 for all other platforms
        $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
            features = (
                export_name = $export_name,
                link_section = $link_section,
                priority = 65535,
                used_linker_meta = $used_linker_meta,
            ),
            meta = $meta,
            unsafe = $unsafe,
            item = $item
        ));
    };

    ( @entry next=$next:path[$next_args:tt], input=(
        features = (
            export_name = $export_name:tt,
            link_section = $link_section:tt,
            priority = $priority:tt,
            used_linker_meta = $used_linker_meta:tt,
        ),
        meta = $meta:tt,
        unsafe = $unsafe:tt,
        item = $item:tt
    ) ) => {
        #[cfg(target_vendor = "apple")]
        $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
            link_args = (
                export_name = $export_name,
                priority = $priority,
                used = $used_linker_meta,
            ),
            meta = $meta,
            unsafe = $unsafe,
            item = $item
        ));

        // Get a priority literal
        #[cfg(not(target_vendor = "apple"))]
        $crate::__priority_to_literal!($crate::__ctor_parse_impl,[
            @priority next=$next[$next_args],
            features = (
                export_name = $export_name,
                link_section = $link_section,
                used_linker_meta = $used_linker_meta,
            ),
            meta = $meta,
            unsafe = $unsafe,
            item = $item
        ] = $priority);
    };

    ( [@priority next=$next:path[$next_args:tt],
        features = (
            export_name = (),
            link_section = $link_section:tt,
            used_linker_meta = $used_linker_meta:tt,
        ),
        meta = $meta:tt,
        unsafe = $unsafe:tt,
        item = $item:tt
    ], ($($priority:tt)*)) => {
        $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
            link_args = (
                export_name = (),
                link_section = (concat!($link_section, ".", $($priority)*)),
                used = $used_linker_meta,
            ),
            meta = $meta,
            unsafe = $unsafe,
            item = $item
        ));
    };

    ( [@priority next=$next:path[$next_args:tt],
        features = (
            export_name = ($($prefix:tt)*, $($suffix:tt)*),
            link_section = $link_section:tt,
            used_linker_meta = $used_linker_meta:tt,
        ),
        meta = $meta:tt,
        unsafe = $unsafe:tt,
        item = $item:tt
    ], ($($priority:tt)*)) => {
        $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
            link_args = (
                export_name = (concat!($($prefix)*, $($priority)*, $($suffix)*)),
                link_section = (concat!($link_section, ".", $($priority)*)),
                used = $used_linker_meta,
            ),
            meta = $meta,
            unsafe = $unsafe,
            item = $item
        ));
    };

    // Step 8: Delegate on item type
    ( @entry next=$next:path[$next_args:tt], input=(
        link_args = $link_args:tt,
        meta = ($($meta:tt)*),
        unsafe = ($($unsafe:tt)*),
        item = ($vis:vis $(unsafe)? $( extern $abi:literal )? fn $name:ident () $( -> () )? {
            $($body:tt)*
        })
    ) ) => {
        $($meta)*
        #[allow(dead_code)]
        $vis $($unsafe)* $( extern $abi )? fn $name () {
            // The outer function may be attached to a struct, so we generate an
            // inner function that is freestanding and call it from both places.
            $($unsafe)* $( extern $abi )? fn __ctor_private_inner() {
                $($body)*
            }

            $crate::__ctor_parse_impl!(@ctor $link_args body={ $($unsafe)* { __ctor_private_inner() } });
            $($unsafe)* { __ctor_private_inner() }
        }
    };

    ( @entry next=$next:path[$next_args:tt], input=(
        link_args = $link_args:tt,
        meta = ($($meta:tt)*),
        unsafe = ($($unsafe:tt)*),
        item = ($vis:vis static $ident:ident : $ty:ty = $(unsafe)? { $($body:tt)* };)
    ) ) => {
        $($meta)*
        $vis static $ident: $crate::statics::Static<$ty> = {
            fn init() -> $ty {
                $($unsafe)* {$($body)*}
            }
            unsafe { $crate::statics::Static::<$ty>::new(init) }
        };
        $crate::__ctor_parse_impl!(@ctor $link_args body={ _ = &*$ident } );
    };

    // ctor definitions
    ( @ctor (
        export_name=(),
        link_section=($($link_section:tt)*),
        used=(#$used_linker_meta:tt),
     ) body=$body:tt ) => {
        const _: () = {
            #[allow(unsafe_code)]
            #[cfg_attr(clippy, allow(unknown_lints, unsafe_attr_outside_unsafe))]
            #[link_section = $($link_section)*]
            #$used_linker_meta
            static __CTOR_PRIVATE_REF: unsafe extern "C" fn() = {
                #[allow(unused_unsafe)]
                extern "C" fn __ctor_private() {
                    $body
                }
                __ctor_private
            };
        };
    };

    ( @ctor (
        export_name=(),
        priority=$priority:tt,
        used=(#$used_linker_meta:tt),
     ) body=$body:tt ) => {
        const _: () = {
            #[allow(unsafe_code, unused_unsafe)]
            extern "C" fn __ctor_private() {
                $body
            }

            $crate::__register_ctor!(priority = $priority, fn = __ctor_private);
        };
    };

    ( @ctor (
        export_name=($($link_name:tt)*),
        link_section=$link_section:tt,
        used=(#$used_linker_meta:tt),
     ) body=$body:tt ) => {
        const _: () = {
            #[allow(unsafe_code)]
            #[cfg_attr(clippy, allow(unknown_lints, unsafe_attr_outside_unsafe))]
            const _: () = {
                #[allow(unused_unsafe)]
                #[no_mangle]
                #[export_name = $($link_name)*]
                extern "C" fn __ctor_private() {
                    $body
                }
                __ctor_private
            };
        };
    };
}
