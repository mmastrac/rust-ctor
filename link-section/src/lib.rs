#![doc = include_str!("../README.md")]
#![allow(unsafe_code)]

pub mod declarative {
    pub use crate::__section_parse as section;
    pub use crate::__in_section_parse as in_section;
}

#[doc(hidden)]
pub mod __support {
    pub use crate::__in_section_crate as in_section_crate;
    pub use crate::__in_section_parse as in_section_parse;
    pub use crate::__section_parse as section_parse;
    pub use crate::__def_section_name as def_section_name;
    pub use crate::__add_section_link_attribute as add_section_link_attribute;

    #[cfg(feature = "proc_macro")]
    pub use link_section_proc_macro::hash;
    #[cfg(feature = "proc_macro")]
    pub use link_section_proc_macro::ident_concat;

    /// Declares the section_name macro.
    #[macro_export]
    #[doc(hidden)]
    macro_rules! __def_section_name {
        (
            {$(
                $__section:ident $__type:ident => $__prefix:tt __ $__suffix:tt;
            )*}

            MAX_LENGTH = $__max_length:literal;
            HASH_LENGTH = $__hash_length:literal;
            VALID_SECTION_CHARS = $__valid_section_chars:literal;
        ) => {
            /// Internal macro for generating a section name.
            #[macro_export]
            #[doc(hidden)]
            macro_rules! __section_name {
                $(
                    (raw $__section $__type $name:ident) => {
                        concat!(concat! $__prefix, stringify!($name), concat! $__suffix);
                    };
                    ($pattern:tt $__section $__type $name:ident) => {
                        $crate::__support::hash!($pattern $name ($__prefix) ($__suffix) $__hash_length $__max_length $__valid_section_chars);
                    };
                )*
                ($pattern:tt $unknown_section:ident $unknown_type:ident) => {
                    const _: () = {
                        compile_error!("Unknown section type: `{}`/`{}`", stringify!($unknown_section), stringify!($unknown_type));
                    };
                };
            }
        };
    }

    #[cfg(feature = "proc_macro")]
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __add_section_link_attribute(
        ($section:ident $type:ident $name:ident #[$attr:ident = __] $item:item) => {
            $crate::__section_name!(
                (#[$attr = __] $item)
                $section $type $name
            );
        }
    );

    #[cfg(not(feature = "proc_macro"))]
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __add_section_link_attribute(
        ($section:ident $type:ident $name:ident #[$attr:ident = __] $item:item) => {
            #[$attr = $crate::__section_name!(
                raw $section $type $name
            )] $item
        }
    );

    // \x01: "do not mangle" (ref https://github.com/rust-lang/rust-bindgen/issues/2935)
    #[cfg(target_vendor = "apple")]
    def_section_name! {
        {
            data bare => ("__DATA,") __ ();
            code bare => ("__TEXT,") __ ();
            data section => ("__DATA,") __ (",regular,no_dead_strip");
            code section => ("__TEXT,") __ (",regular,no_dead_strip");
            data start => ("\x01section$start$__DATA$") __ ();
            data end => ("\x01section$end$__DATA$") __ ();
        }
        MAX_LENGTH = 16;
        HASH_LENGTH = 6;
        VALID_SECTION_CHARS = "_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    }

    #[cfg(target_family = "wasm")]
    def_section_name! {
        {
            data bare => (".data", ".link_section.") __ ();
            data section => (".data", ".link_section.") __ ();
            code bare => (".text", ".link_section.") __ ();
            code section => (".text", ".link_section.") __ ();
        }
        MAX_LENGTH = 16;
        HASH_LENGTH = 6;
        VALID_SECTION_CHARS = "_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    }

    #[cfg(all(
        not(target_vendor = "apple"),
        not(target_vendor = "pc"),
        not(target_family = "wasm")
    ))]
    def_section_name! {
        {
            data bare => ("_data", "_link_section_") __ ();
            data section => ("_data", "_link_section_") __ ();
            data start => ("__start_", "_data", "_link_section_") __ ();
            data end => ("__stop_", "_data", "_link_section_") __ ();
            code bare => ("_text", "_link_section_") __ ();
            code section => ("_text", "_link_section_") __ ();
            code start => ("__start_", "_text", "_link_section_") __ ();
            code end => ("__stop_", "_text", "_link_section_") __ ();
        }
        MAX_LENGTH = 64;
        HASH_LENGTH = 10;
        VALID_SECTION_CHARS = "_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    }

    #[cfg(target_vendor = "pc")]
    def_section_name! {
        {
            data bare => (".data", "$") __ ();
            data section => (".data", "$") __ ("$b");
            data start => (".data", "$") __ ("$a");
            data end => (".data", "$") __ ("$c");
            code bare => (".text", "$") __ ();
            code section => (".text", "$") __ ("$b");
            code start => (".text", "$") __ ("$a");
            code end => (".text", "$") __ ("$c");
        }
        MAX_LENGTH = 64;
        HASH_LENGTH = 10;
        VALID_SECTION_CHARS = "_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    }

    /// Define a link section.
    #[macro_export]
    #[doc(hidden)]
    macro_rules! __section_parse {
        // Has a generic (note that $generic eats the trailing semicolon)
        (#[section] $(#[$meta:meta])* $vis:vis static $ident:ident : $(:: $path_prefix:ident ::)? $($path:ident)::* < $($generic:tt)*) => {
            $crate::__section_parse!(#[section] $(#[$meta])* $vis static $ident: ( $(:: $path_prefix ::)? $($path)::* < $($generic)*) ( $($generic)* ) generic);
        };
        // No generic
        (#[section] $(#[$meta:meta])* $vis:vis static $ident:ident : $(:: $path_prefix:ident ::)? $($path:ident)::* ;) => {
            $crate::__section_parse!(#[section] $(#[$meta])* $vis static $ident: ( $(:: $path_prefix ::)? $($path)::* ;) ( () > ; ) no_generic);
        };
        // Both end up here...
        (#[section] $(#[$meta:meta])* $vis:vis static $ident:ident : ($ty:ty ;) ( $generic_ty:ty > ; ) $generic:ident) => {
            /// Internal macro for parsing the section.
            macro_rules! $ident {
                (v=0 (item=$item:tt $rest:tt)) => {
                    $crate::__support::in_section_crate!($ident $generic $ty, $item);
                };
                (v=$v:literal $rest:tt) => {
                    const _: () = { compile_error!(concat!("link-section: Unsupported version: `", stringify!($v), "`")); };
                };
            }

            $(#[$meta])*
            #[used]
            #[cfg(target_family = "wasm")]
            $vis static $ident: $crate::__support::Section< $ty, $generic_ty > = {
                static __START: ::core::sync::atomic::AtomicPtr::<::core::marker::PhantomData<$generic_ty>> = unsafe {
                    ::core::sync::atomic::AtomicPtr::<::core::marker::PhantomData<$generic_ty>>::new(::core::ptr::null_mut())
                };
                static __END: ::core::sync::atomic::AtomicPtr::<::core::marker::PhantomData<$generic_ty>> = unsafe {
                    ::core::sync::atomic::AtomicPtr::<::core::marker::PhantomData<$generic_ty>>::new(::core::ptr::null_mut())
                };

                $crate::__support::ident_concat!((#[no_mangle]pub extern "C" fn) (register_link_section_ $ident) ((data_ptr: *const u8, data_len: usize) {
                    unsafe {
                        __START.store(data_ptr as *mut ::core::marker::PhantomData<$generic_ty>, ::core::sync::atomic::Ordering::Relaxed);
                        __END.store(data_ptr.add(data_len) as *mut ::core::marker::PhantomData<$generic_ty>, ::core::sync::atomic::Ordering::Relaxed);
                    }
                }));

                $crate::__support::Section::new(
                    stringify!($ident),
                    &__START,
                    &__END,
                )
            };

            $(#[$meta])*
            #[used]
            #[cfg(not(target_family = "wasm"))]
            $vis static $ident: $crate::__support::Section< $ty, $generic_ty > = $crate::__support::Section::new(
                {
                    $crate::__section_name!(
                        raw data bare $ident
                    )
                },
                {
                    #[cfg(not(target_vendor = "pc"))]
                    extern "C" {
                        $crate::__support::add_section_link_attribute!(
                            data start $ident
                            #[link_name = __]
                            static __START: $crate::__support::SectionPtr<$generic_ty>;
                        );
                    }

                    // Windows always sorts, so we can use alphabetical order
                    #[cfg(target_vendor = "pc")]
                    $crate::__support::add_section_link_attribute!(
                        data start $ident
                        #[link_name = __]
                        static __START: [$generic_ty; 0] = [];
                    );

                    unsafe { &raw const __START as $crate::__support::SectionPtr<$generic_ty> }
                },
                {
                    #[cfg(not(target_vendor = "pc"))]
                    extern "C" {
                        $crate::__support::add_section_link_attribute!(
                            data end $ident
                            #[link_name = __]
                            static __END: $crate::__support::SectionPtr<$generic_ty>;
                        );
                    }

                    #[cfg(target_vendor = "pc")]
                    $crate::__support::add_section_link_attribute!(
                        data end $ident
                        #[link_name = __]
                        static __END: [$generic_ty; 0] = [];
                    );

                    unsafe { &raw const __END as $crate::__support::SectionPtr<$generic_ty> }
                },
            );
        };
    }

    /// Export a symbol into a link section.
    #[macro_export]
    #[doc(hidden)]
    macro_rules! __in_section_parse {
        (#[in_section($name:path)] $($item:tt)*) => {
            $name ! (
                v=0 (item=($($item)*) ())
            );
        };
    }

    #[macro_export]
    #[doc(hidden)]
    #[allow(unknown_lints, edition_2024_expr_fragment_specifier)]
    macro_rules! __in_section_crate {
        ($ident:ident generic $section_ty:ty, ($(#[$meta:meta])* $vis:vis fn $ident_fn:ident($($args:tt)*) $(-> $ret:ty)? $body:block)) => {
            $crate::__section_name!(
                (
                    // Split the function into a static item and a function pointer
                    $(#[$meta])*
                    #[used]
                    #[link_section = __]
                    #[allow(non_upper_case_globals)]
                    $vis static $ident_fn: <$section_ty as $crate::__support::SectionItemType>::Item =
                        {
                            fn $ident_fn($($args)*) $(-> $ret)? $body
                            $ident_fn as <$section_ty as $crate::__support::SectionItemType>::Item
                        };
                )
                data section $ident
            );
        };
        ($ident:ident generic $section_ty:ty, ($(#[$meta:meta])* $vis:vis static $ident_static:ident : $ty:ty = $value:expr;)) => {
            $crate::__section_name!(
                ($(#[$meta])* #[link_section = __] #[used] $vis static $ident_static: <$section_ty as $crate::__support::SectionItemType>::Item = $value;)
                data section $ident
            );
        };
        ($ident:ident no_generic $section_ty:ty, ($(#[$meta:meta])* $vis:vis fn $ident_fn:ident($($args:tt)*) $(-> $ret:ty)? $body:block)) => {
            $crate::__section_name!(
                (
                    $(#[$meta])*
                    #[link_section = __]
                    #[used]
                    #[allow(non_upper_case_globals)]
                    $vis static $ident_fn: fn($($args)*) $(-> $ret)? =
                        {
                            $crate::__section_name!(
                                (#[link_section = __] fn $ident_fn($($args)*) $(-> $ret)? $body)
                                code section $ident
                            );
                            $ident_fn
                        };
                )
                data section $ident
            );
        };
        ($ident:ident no_generic $section_ty:ty, ($(#[$meta:meta])* $item:item)) => {
            $crate::__section_name!(
                ($(#[$meta])* #[link_section = __] #[used] $item)
                data section $ident
            );
        };
    }

    pub trait SectionItemType {
        type Item;
    }

    #[repr(C)]
    pub struct Section<T: sealed::FromRawSection, S: 'static> {
        name: &'static str,
        start: SectionPtr<S>,
        end: SectionPtr<S>,
        _t: ::core::marker::PhantomData<T>,
    }

    impl<T> SectionItemType for super::TypedSection<T> {
        type Item = T;
    }

    impl<T: sealed::FromRawSection, S> Section<T, S> {
        pub const fn new(name: &'static str, start: SectionPtr<S>, end: SectionPtr<S>) -> Self {
            Self {
                name,
                start,
                end,
                _t: ::core::marker::PhantomData,
            }
        }
    }

    impl<'a, T: sealed::FromRawSection, S> ::core::iter::IntoIterator for &'a Section<T, S>
    where
        for<'b> &'b T: ::core::iter::IntoIterator,
    {
        type Item = <&'a T as ::core::iter::IntoIterator>::Item;
        type IntoIter = <&'a T as ::core::iter::IntoIterator>::IntoIter;
        fn into_iter(self) -> Self::IntoIter {
            ::core::ops::Deref::deref(self).into_iter()
        }
    }

    impl<T: sealed::FromRawSection, S> ::core::ops::Deref for Section<T, S> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            // SAFETY: all sections are repr(C)
            unsafe { ::core::mem::transmute(self) }
        }
    }

    impl<T: sealed::FromRawSection + ::core::fmt::Debug, S> ::core::fmt::Debug for Section<T, S> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            ::core::fmt::Debug::fmt(::core::ops::Deref::deref(self), f)
        }
    }

    unsafe impl<T: sealed::FromRawSection, S> Sync for Section<T, S> {}
    unsafe impl<T: sealed::FromRawSection, S> Send for Section<T, S> {}

    /// On Apple platforms, the linker provides a pointer to the start and end
    /// of the section regardless of the section's name.
    #[cfg(all(not(target_vendor = "pc"), not(target_family = "wasm")))]
    pub type SectionPtr<T> = *const ::core::marker::PhantomData<T>;
    /// On LLVM/GCC/MSVC platforms, we cannot use start/end symbols for sections
    /// without C-compatible names, so instead we drop a [T; 0] at the start and
    /// end of the section.
    #[cfg(target_vendor = "pc")]
    pub type SectionPtr<T> = *const [T; 0];
    /// On WASM, we use an atomic pointer to the start and end of the section.
    #[cfg(target_family = "wasm")]
    pub type SectionPtr<T> =
        &'static ::core::sync::atomic::AtomicPtr<::core::marker::PhantomData<T>>;

    mod sealed {
        pub trait FromRawSection {}

        impl FromRawSection for crate::Section {}

        impl<T> FromRawSection for crate::TypedSection<T> {}
    }
}

/// Define a link section.
///
/// # Example
/// ```rust
/// use link_section::{in_section, section};
///
/// #[section]
/// pub static DATA_SECTION: link_section::Section;
///
/// #[in_section(DATA_SECTION)]
/// pub fn data_function() {
///     println!("data_function");
/// }
/// ```
#[cfg(feature = "proc_macro")]
pub use ::link_section_proc_macro::section;

/// Place an item into a link section.
///
/// # Functions and typed sections
///
/// As a special case, since function declarations by themselves are not sized,
/// functions in typed sections are split and stored as function pointers.
#[cfg(feature = "proc_macro")]
pub use ::link_section_proc_macro::in_section;

/// An untyped link section that can be used to store any type. The underlying
/// data is not enumerable.
#[repr(C)]
pub struct Section {
    name: &'static str,
    start: __support::SectionPtr<()>,
    end: __support::SectionPtr<()>,
}

#[cfg(target_family = "wasm")]
impl Section {
    /// The start address of the section.
    pub fn start_ptr(&self) -> *const () {
        let ptr = self.start.load(::core::sync::atomic::Ordering::Relaxed) as *const ();
        if ptr.is_null() {
            panic!(
                "Section {} was not initialized by the host environment",
                self.name
            );
        }
        ptr
    }
    /// The end address of the section.
    pub fn end_ptr(&self) -> *const () {
        let ptr = self.end.load(::core::sync::atomic::Ordering::Relaxed) as *const ();
        if ptr.is_null() {
            panic!(
                "Section {} was not initialized by the host environment",
                self.name
            );
        }
        ptr
    }
    /// The byte length of the section.
    pub fn byte_len(&self) -> usize {
        unsafe { (self.end_ptr() as *const u8).offset_from(self.start_ptr() as *const u8) as usize }
    }
}

#[cfg(not(target_family = "wasm"))]
impl Section {
    /// The start address of the section.
    pub const fn start_ptr(&self) -> *const () {
        self.start as *const ()
    }
    /// The end address of the section.
    pub const fn end_ptr(&self) -> *const () {
        self.end as *const ()
    }
    /// The byte length of the section.
    pub const fn byte_len(&self) -> usize {
        unsafe { (self.end_ptr() as *const u8).offset_from(self.start_ptr() as *const u8) as usize }
    }
}

impl ::core::fmt::Debug for Section {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Section")
            .field("name", &self.name)
            .field("start", &self.start_ptr())
            .field("end", &self.end_ptr())
            .field("byte_len", &self.byte_len())
            .finish()
    }
}

unsafe impl Sync for Section {}
unsafe impl Send for Section {}

/// A typed link section that can be used to store any sized type. The
/// underlying data is enumerable.
#[repr(C)]
pub struct TypedSection<T: 'static> {
    name: &'static str,
    start: __support::SectionPtr<T>,
    end: __support::SectionPtr<T>,
    _phantom: ::core::marker::PhantomData<T>,
}

#[cfg(target_family = "wasm")]
impl<T: 'static> TypedSection<T> {
    /// The stride of the typed section.
    pub const fn stride(&self) -> usize {
        // Compute the size required for C to store two instances of T side-by-side.
        // TODO: Can we just use align_of/size_of?
        #[repr(C)]
        struct Sizer<T> {
            t1: T,
            t2: T,
            t3: T,
        }

        let sizer = ::core::mem::MaybeUninit::<Sizer<T>>::uninit();
        let ptr: *const Sizer<T> = sizer.as_ptr();
        let start = ptr as *const u8;
        let end = unsafe { ::core::ptr::addr_of!((*ptr).t3) } as *const u8;
        unsafe { end.offset_from(start) as usize / 2 }
    }

    /// The start address of the section.
    pub fn start_ptr(&self) -> *const T {
        let ptr = self.start.load(::core::sync::atomic::Ordering::Relaxed) as *const T;
        if ptr.is_null() {
            panic!(
                "TypedSection {} was not initialized by the host environment",
                self.name
            );
        }
        ptr
    }

    /// The end address of the section.
    pub fn end_ptr(&self) -> *const T {
        let ptr = self.end.load(::core::sync::atomic::Ordering::Relaxed) as *const T;
        if ptr.is_null() {
            panic!(
                "TypedSection {} was not initialized by the host environment",
                self.name
            );
        }
        ptr
    }

    /// The byte length of the section.
    pub fn byte_len(&self) -> usize {
        unsafe { (self.end_ptr() as *const u8).offset_from(self.start_ptr() as *const u8) as usize }
    }

    /// The number of elements in the section.
    pub fn len(&self) -> usize {
        self.byte_len() / self.stride()
    }

    /// True if the section is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// The section as a slice.
    pub fn as_slice(&self) -> &[T] {
        if self.is_empty() {
            &[]
        } else {
            unsafe { ::core::slice::from_raw_parts(self.start_ptr(), self.len()) }
        }
    }
}

#[cfg(not(target_family = "wasm"))]
impl<T: 'static> TypedSection<T> {
    /// The stride of the typed section.
    pub const fn stride(&self) -> usize {
        // Compute the size required for C to store two instances of T side-by-side.
        // TODO: Can we just use align_of/size_of?
        #[repr(C)]
        struct Sizer<T> {
            t1: T,
            t2: T,
            t3: T,
        }

        let sizer = ::core::mem::MaybeUninit::<Sizer<T>>::uninit();
        let ptr: *const Sizer<T> = sizer.as_ptr();
        let start = ptr as *const u8;
        let end = unsafe { ::core::ptr::addr_of!((*ptr).t3) } as *const u8;
        unsafe { end.offset_from(start) as usize / 2 }
    }

    /// The start address of the section.
    pub const fn start_ptr(&self) -> *const T {
        self.start as *const T
    }

    /// The end address of the section.
    pub const fn end_ptr(&self) -> *const T {
        self.end as *const T
    }

    /// The byte length of the section.
    pub const fn byte_len(&self) -> usize {
        unsafe { (self.end_ptr() as *const u8).offset_from(self.start_ptr() as *const u8) as usize }
    }

    /// The number of elements in the section.
    pub const fn len(&self) -> usize {
        self.byte_len() / self.stride()
    }

    /// True if the section is empty.
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// The section as a slice.
    pub const fn as_slice(&self) -> &[T] {
        if self.is_empty() {
            &[]
        } else {
            unsafe { ::core::slice::from_raw_parts(self.start_ptr(), self.len()) }
        }
    }
}

impl<'a, T> ::core::iter::IntoIterator for &'a TypedSection<T> {
    type Item = &'a T;
    type IntoIter = ::core::slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.as_slice().iter()
    }
}

impl<T> ::core::ops::Deref for TypedSection<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T> ::core::fmt::Debug for TypedSection<T> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TypedSection")
            .field("name", &self.name)
            .field("start", &self.start_ptr())
            .field("end", &self.end_ptr())
            .field("len", &self.len())
            .field("stride", &self.stride())
            .finish()
    }
}

unsafe impl<T> Sync for TypedSection<T> where T: Sync {}
unsafe impl<T> Send for TypedSection<T> where T: Send {}
