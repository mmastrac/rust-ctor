#[doc(hidden)]
pub mod __support {
    pub use crate::__in_section_crate as in_section_crate;
    pub use crate::__in_section_parse as in_section_parse;
    pub use crate::__section_name as section_name;
    pub use crate::__section_parse as section_parse;

    pub use link_section_proc_macro::hash;

    #[cfg(target_vendor = "apple")]
    #[macro_export]
    #[doc(hidden)]
    macro_rules! __section_name {
        ($pattern:tt data $($rest:tt)*) => {
            $crate::__support::section_name!(__ $pattern symbol "__DATA" $($rest)*);
        };
        ($pattern:tt code $($rest:tt)*) => {
            $crate::__support::section_name!(__ $pattern symbol "__TEXT" $($rest)*);
        };
        ($pattern:tt $unknown_section:ident $($rest:tt)*) => {
            const _: () = {
                compile_error!("Unknown section type: `{}`", stringify!($unknown_section));
            };
        };

        (__ $pattern:tt symbol $section_prefix:literal bare $name:ident) => {
            $crate::__support::section_name!(__ $pattern hash ($section_prefix ",") () $name);
        };
        (__ $pattern:tt symbol $section_prefix:literal section $name:ident) => {
            $crate::__support::section_name!(__ $pattern hash ($section_prefix ",") (",regular,no_dead_strip")$name);
        };
        (__ $pattern:tt symbol $section_prefix:literal fn_body $name:ident) => {
            $crate::__support::section_name!(__ $pattern hash ($section_prefix ",") (",regular,pure_instructions")$name);
        };
        (__ $pattern:tt symbol $section_prefix:literal start $name:ident) => {
            // \x01: "do not mangle" (ref https://github.com/rust-lang/rust-bindgen/issues/2935)
            $crate::__support::section_name!(__ $pattern hash ("\x01section$start$" $section_prefix "$") () $name);
        };
        (__ $pattern:tt symbol $section_prefix:literal end $name:ident) => {
            $crate::__support::section_name!(__ $pattern hash ("\x01section$end$" $section_prefix "$") ()$name);
        };

        (__ $pattern:tt hash $prefix:tt $suffix:tt $name:ident) => {
            $crate::__support::hash!($pattern $name $prefix $suffix 6 16 "_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789");
        };
    }

    #[cfg(all(
        target_os = "linux",
    ))]
    #[macro_export]
    #[doc(hidden)]
    macro_rules! __section_name {
        ($pattern:tt data $($rest:tt)*) => {
            $crate::__support::section_name!(__ $pattern symbol ".data" $($rest)*);
        };
        ($pattern:tt code $($rest:tt)*) => {
            $crate::__support::section_name!(__ $pattern symbol ".text" $($rest)*);
        };
        ($pattern:tt $unknown_section:ident $($rest:tt)*) => {
            const _: () = {
                compile_error!("Unknown section type: `{}`", stringify!($unknown_section));
            };
        };

        (__ $pattern:tt symbol $section_prefix:literal bare $name:ident) => {
            $crate::__support::section_name!(__ $pattern hash ($section_prefix ".") () $name);
        };
        (__ $pattern:tt symbol $section_prefix:literal section $name:ident) => {
            $crate::__support::section_name!(__ $pattern hash ($section_prefix ".") () $name);
        };
        (__ $pattern:tt symbol $section_prefix:literal fn_body $name:ident) => {
            $crate::__support::section_name!(__ $pattern hash ($section_prefix ".") () $name);
        };
        (__ $pattern:tt symbol $section_prefix:literal start $name:ident) => {
            $crate::__support::section_name!(__ $pattern hash ("__start_" $section_prefix ",") () $name);
        };
        (__ $pattern:tt symbol $section_prefix:literal end $name:ident) => {
            $crate::__support::section_name!(__ $pattern hash ("__stop_" $section_prefix ",") () $name);
        };

        (__ $pattern:tt hash $prefix:tt $suffix:tt $name:ident) => {
            $crate::__support::hash!($pattern $name $prefix $suffix 10 64 "_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789");
        };
    }

    #[cfg(target_vendor = "pc")]
    #[macro_export]
    #[doc(hidden)]
    macro_rules! __section_name {
        ($pattern:tt data $($rest:tt)*) => {
            $crate::__support::section_name!(__ $pattern symbol ".data" $($rest)*);
        };
        ($pattern:tt code $($rest:tt)*) => {
            $crate::__support::section_name!(__ $pattern symbol ".text" $($rest)*);
        };
        ($pattern:tt $unknown_section:ident $($rest:tt)*) => {
            const _: () = {
                compile_error!("Unknown section type: `{}`", stringify!($unknown_section));
            };
        };

        (__ $pattern:tt symbol $section_prefix:literal bare $name:ident) => {
            $crate::__support::section_name!(__ $pattern hash ($section_prefix "$") ("$b") $name);
        };
        (__ $pattern:tt symbol $section_prefix:literal section $name:ident) => {
            $crate::__support::section_name!(__ $pattern hash ($section_prefix "$") ("$b") $name);
        };
        (__ $pattern:tt symbol $section_prefix:literal fn_body $name:ident) => {
            $crate::__support::section_name!(__ $pattern hash ($section_prefix "$") ("$d") $name);
        };
        (__ $pattern:tt symbol $section_prefix:literal start $name:ident) => {
            $crate::__support::section_name!(__ $pattern hash ("__start_" $section_prefix ",") ("$a") $name);
        };
        (__ $pattern:tt symbol $section_prefix:literal end $name:ident) => {
            $crate::__support::section_name!(__ $pattern hash ("__stop_" $section_prefix ",") ("$c") $name);
        };

        (__ $pattern:tt hash $prefix:tt $suffix:tt $name:ident) => {
            $crate::__support::hash!($pattern $name $prefix $suffix 10 64 "_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789");
        };
    }

    /// Define a link section.
    #[macro_export]
    #[doc(hidden)]
    macro_rules! __section_parse {
        // Has a generic (note that $generic eats the trailing semicolon)
        (#[section($section:ident)] $vis:vis static $ident:ident : $(:: $path_prefix:ident ::)? $($path:ident)::* < $($generic:tt)*) => {
            $crate::__section_parse!(#[section($section)] $vis static $ident: ( $(:: $path_prefix ::)? $($path)::* < $($generic)*) generic);
        };
        // No generic
        (#[section($section:ident)] $vis:vis static $ident:ident : $(:: $path_prefix:ident ::)? $($path:ident)::* ;) => {
            $crate::__section_parse!(#[section($section)] $vis static $ident: ( $(:: $path_prefix ::)? $($path)::* ;) no_generic);
        };
        // Both end up here...
        (#[section($section:ident)] $vis:vis static $ident:ident : ($ty:ty ;) $generic:ident) => {
            macro_rules! $ident {
                (item=$item:tt) => {
                    $crate::__support::in_section_crate!($ident $generic $section $ty, $item);
                };
            }

            $vis static $ident: $crate::__support::Section< $ty > = $crate::__support::Section::new(
                {
                    $crate::__support::section_name!(
                        (const fn section_name() -> &'static str { __ })
                        $section bare $ident
                    );

                    section_name()
                },
                {
                    $crate::__support::section_name!(
                        (
                            extern "C" {
                                #[link_name = __] static __START: $crate::__support::SectionPtr;
                            }
                        )
                        $section start $ident
                    );

                    unsafe { &raw const __START as $crate::__support::SectionPtr }
                },
                {
                    $crate::__support::section_name!(
                        (
                            extern "C" {
                                #[link_name = __] static __END: $crate::__support::SectionPtr;
                            }
                        )
                        $section end $ident
                    );

                    unsafe { &raw const __END as $crate::__support::SectionPtr }
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
                item=($($item)*)
            );
        };
    }

    #[macro_export]
    #[doc(hidden)]
    macro_rules! __in_section_crate {
        ($ident:ident generic $section:ident $section_ty:ty, ($vis:vis fn $ident_fn:ident($($args:tt)*) $(-> $ret:ty)? $body:block)) => {
            $crate::__support::section_name!(
                (
                    // Split the function into a static item and a function pointer
                    #[no_mangle]
                    #[link_section = __]
                    $vis static $ident_fn: <$section_ty as $crate::__support::SectionItemType>::Item =
                        {
                            fn $ident_fn($($args)*) $(-> $ret)? $body
                            $ident_fn as <$section_ty as $crate::__support::SectionItemType>::Item
                        };
                )
                $section section $ident
            );
        };
        ($ident:ident generic $section:ident $section_ty:ty, ($vis:vis static $ident_static:ident : $ty:ty = $value:expr;)) => {
            $crate::__support::section_name!(
                (#[no_mangle] #[link_section = __] $vis static $ident_static: <$section_ty as $crate::__support::SectionItemType>::Item = $value;)
                $section section $ident
            );
        };
        ($ident:ident no_generic $section:ident $section_ty:ty, ($vis:vis fn $ident_fn:ident($($args:tt)*) $(-> $ret:ty)? $body:block)) => {
            $crate::__support::section_name!(
                (
                    #[no_mangle]
                    #[link_section = __]
                    $vis fn $ident_fn($($args)*) $(-> $ret)? $body
                )
                $section fn_body $ident
            );
        };
        ($ident:ident no_generic $section:ident $section_ty:ty, ($item:item)) => {
            $crate::__support::section_name!(
                (#[no_mangle] #[link_section = __] $item)
                $section section $ident
            );
        };
    }

    pub trait SectionItemType {
        type Item;
    }

    #[repr(C)]
    pub struct Section<T: sealed::FromRawSection> {
        name: &'static str,
        start: SectionPtr,
        end: SectionPtr,
        _t: ::core::marker::PhantomData<T>,
    }

    impl<T> SectionItemType for super::TypedSection<T> {
        type Item = T;
    }

    impl<T: sealed::FromRawSection> Section<T> {
        pub const fn new(name: &'static str, start: SectionPtr, end: SectionPtr) -> Self {
            Self {
                name,
                start,
                end,
                _t: ::core::marker::PhantomData,
            }
        }
    }

    impl<'a, T: sealed::FromRawSection> ::core::iter::IntoIterator for &'a Section<T>
    where
        for<'b> &'b T: ::core::iter::IntoIterator,
    {
        type Item = <&'a T as ::core::iter::IntoIterator>::Item;
        type IntoIter = <&'a T as ::core::iter::IntoIterator>::IntoIter;
        fn into_iter(self) -> Self::IntoIter {
            ::core::ops::Deref::deref(self).into_iter()
        }
    }

    impl<T: sealed::FromRawSection> ::core::ops::Deref for Section<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            unsafe { ::core::mem::transmute(self) }
        }
    }

    impl<T: sealed::FromRawSection + ::core::fmt::Debug> ::core::fmt::Debug for Section<T> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            ::core::fmt::Debug::fmt(::core::ops::Deref::deref(self), f)
        }
    }

    unsafe impl<T: sealed::FromRawSection> Sync for Section<T> {}
    unsafe impl<T: sealed::FromRawSection> Send for Section<T> {}

    pub type SectionPtr = *const ();

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
/// #[section(data)]
/// pub static DATA_SECTION: link_section::Section;
///
/// #[in_section(DATA_SECTION)]
/// pub fn data_function() {
///     println!("data_function");
/// }
/// ```
pub use ::link_section_proc_macro::section;

/// Place an item into a link section.
///
/// # Functions and typed sections
///
/// As a special case, since function declarations by themselves are not sized,
/// functions in typed sections are split and stored as function pointers.
pub use ::link_section_proc_macro::in_section;

/// An untyped link section that can be used to store any type. The underlying
/// data is not enumerable.
#[repr(C)]
pub struct Section {
    name: &'static str,
    start: __support::SectionPtr,
    end: __support::SectionPtr,
}

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
        unsafe { (self.end as *const u8).offset_from(self.start as *const u8) as usize }
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
pub struct TypedSection<T> {
    name: &'static str,
    start: __support::SectionPtr,
    end: __support::SectionPtr,
    _phantom: ::core::marker::PhantomData<T>,
}

impl<T> TypedSection<T> {
    /// The stride of the typed section.
    pub const fn stride() -> usize {
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
        let size = unsafe { end.offset_from(start) as usize / 2 };
        size
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
        unsafe { (self.end as *const u8).offset_from(self.start as *const u8) as usize }
    }

    /// The number of elements in the section.
    pub const fn len(&self) -> usize {
        Self::byte_len(&self) / Self::stride()
    }

    /// The section as a slice.
    pub const fn as_slice(&self) -> &[T] {
        if self.len() == 0 {
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
        self.as_slice().into_iter()
    }
}

impl<T> ::core::ops::Deref for TypedSection<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        &self.as_slice()
    }
}

impl<T> ::core::fmt::Debug for TypedSection<T> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TypedSection")
            .field("name", &self.name)
            .field("start", &self.start_ptr())
            .field("end", &self.end_ptr())
            .field("len", &self.len())
            .field("stride", &Self::stride())
            .finish()
    }
}

unsafe impl<T> Sync for TypedSection<T> where T: Sync {}
unsafe impl<T> Send for TypedSection<T> where T: Send {}
