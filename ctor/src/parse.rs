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
            link_name_prefix = $link_name_prefix:tt,
            link_section = $link_section:tt,
            no_warn_on_missing_unsafe = $no_warn_on_missing_unsafe:tt,
            priority = $priority:tt,
            proc_macro = $proc_macro:tt,
            std = $std:tt,
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
                link_name_prefix = $link_name_prefix,
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
            crate_path = $crate_path:tt,
            link_name_prefix = $link_name_prefix:tt,
            link_section = $link_section:tt,
            no_warn_on_missing_unsafe = $no_warn_on_missing_unsafe:tt,
            priority = $priority:tt,
            proc_macro = $proc_macro:tt,
            std = $std:tt,
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
            crate_path = $crate_path:tt,
            link_name_prefix = $link_name_prefix:tt,
            link_section = $link_section:tt,
            no_warn_on_missing_unsafe = $no_warn_on_missing_unsafe:tt,
            priority = $priority:tt,
            proc_macro = $proc_macro:tt,
            std = $std:tt,
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
            crate_path = $crate_path:tt,
            link_name_prefix = $link_name_prefix:tt,
            link_section = $link_section:tt,
            no_warn_on_missing_unsafe = $no_warn_on_missing_unsafe:tt,
            priority = $priority:tt,
            proc_macro = $proc_macro:tt,
            std = $std:tt,
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
            crate_path = $crate_path:tt,
            link_name_prefix = $link_name_prefix:tt,
            link_section = $link_section:tt,
            no_warn_on_missing_unsafe = $no_warn_on_missing_unsafe:tt,
            priority = $priority:tt,
            proc_macro = $proc_macro:tt,
            std = $std:tt,
            used_linker = $used_linker:tt,
        ),
        meta = $meta:tt,
        unsafe = ($($unsafe:tt)?),
        item = ($vis:vis static $ident:ident : $ty:ty = $(unsafe)? { $($body:tt)* };)
    ) ) => {
        $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
            features = (
                anonymous = $anonymous,
                link_name_prefix = $link_name_prefix,
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
            crate_path = $crate_path:tt,
            link_name_prefix = $link_name_prefix:tt,
            link_section = $link_section:tt,
            no_warn_on_missing_unsafe = $no_warn_on_missing_unsafe:tt,
            priority = $priority:tt,
            proc_macro = $proc_macro:tt,
            std = $std:tt,
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


    // Step 1: Compute no_warn_on_missing_unsafe

    // warn iff no_warn_on_missing_unsafe is not present AND unsafe is not present
    ( @entry next=$next:path[$next_args:tt], input=(
        features = (
            anonymous = $anonymous:tt,
            link_name_prefix = $link_name_prefix:tt,
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
                link_name_prefix = $link_name_prefix,
                link_section = $link_section,
                priority = $priority,
                used_linker = $used_linker,
            ),
            meta = $meta,
            item = $item
        ));
    };

    ( @entry next=$next:path[$next_args:tt], input=(
        features = (
            anonymous = $anonymous:tt,
            link_name_prefix = $link_name_prefix:tt,
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
                link_name_prefix = $link_name_prefix,
                link_section = $link_section,
                priority = $priority,
                used_linker = $used_linker,
            ),
            meta = $meta,
            item = $item
        ));
    };

    // Step 2: Compute priority
    ( @entry next=$next:path[$next_args:tt], input=(
        features = (
            anonymous = $anonymous:tt,
            link_name_prefix = $link_name_prefix:tt,
            link_section = $link_section:tt,
            priority = (),
            used_linker = $used_linker:tt,
        ),
        meta = $meta:tt,
        item = $item:tt
    ) ) => {
        $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
            features = (
                anonymous = $anonymous,
                link_name = (),
                link_section = ($link_section),
                used_linker = $used_linker,
            ),
            meta = $meta,
            item = $item
        ));
    };

    ( @entry next=$next:path[$next_args:tt], input=(
        features = (
            anonymous = $anonymous:tt,
            link_name_prefix = $link_name_prefix:tt,
            link_section = $link_section:tt,
            priority = $priority:tt,
            used_linker = $used_linker:tt,
        ),
        meta = $meta:tt,
        item = $item:tt
    ) ) => {
        // #[cfg(target_vendor = "apple")]
        // $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
        //     features = (
        //         anonymous = $anonymous,
        //         link_section = ($link_section), //(concat!($link_section, ".", $priority)),
        //         used_linker = $used_linker,
        //     ),
        //     meta = $meta,
        //     item = $item
        // ));

        // #[cfg(not(target_vendor = "apple"))]
        $crate::__priority_to_literal!($crate::__ctor_parse_impl,[
            @priority next=$next[$next_args],
            features = (
                anonymous = $anonymous,
                link_name = (),
                link_section = $link_section,
                used_linker = $used_linker,
            ),
            meta = $meta,
            item = $item
        ] = $priority);
    };

    ( [@priority next=$next:path[$next_args:tt],
        features = (
            anonymous = $anonymous:tt,
            link_name = $link_name:tt,
            link_section = $link_section:tt,
            used_linker = $used_linker:tt,
        ),
        meta = $meta:tt,
        item = $item:tt
    ], ($($priority:tt)*)) => {
        $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
            features = (
                anonymous = $anonymous,
                link_name = $link_name,
                link_section = (concat!($link_section, ".", $($priority)*)),
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
            link_name = $link_name:tt,
            link_section = $link_section:tt,
            used_linker = $used_linker:tt,
        ),
        meta = $meta:tt,
        item = $item:tt
    ) ) => {
        $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
            features = (
                link_name = $link_name,
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
            link_name = $link_name:tt,
            link_section = $link_section:tt,
            used_linker = $used_linker:tt,
        ),
        meta = $meta:tt,
        item = $item:tt
    ) ) => {
        const _: () = {
            $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
                features = (
                    link_name = $link_name,
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
            link_name = $link_name:tt,
            link_section = $link_section:tt,
            used_linker = (),
        ),
        meta = $meta:tt,
        item = $item:tt
    ) ) => {
        $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
            features = (
                link_name = $link_name,
                link_section = $link_section,
                used_linker_meta = (#[used]),
            ),
            meta = $meta,
            item = $item
        ));
    };

    ( @entry next=$next:path[$next_args:tt], input=(
        features = (
            link_name = $link_name:tt,
            link_section = $link_section:tt,
            used_linker = used_linker,
        ),
        meta = $meta:tt,
        item = $item:tt
    ) ) => {
        $crate::__ctor_parse_impl!(@entry next=$next[$next_args], input=(
            features = (
                link_name = $link_name,
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
            link_name = $link_name:tt,
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
            link_name = $link_name:tt,
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
            link_name = $link_name:tt,
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
