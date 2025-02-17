#[doc(hidden)]
#[allow(unused)]
pub mod __support {
    pub use crate::__ctor_entry as ctor_entry;
    pub use crate::__ctor_link_section as ctor_link_section;
    pub use crate::__ctor_link_section_attr as ctor_link_section_attr;
    #[doc(hidden)]
    pub use crate::__ctor_parse as dtor_parse;
    pub use crate::__dtor_entry as dtor_entry;
    pub use crate::__if_has_feature as if_has_feature;
    pub use crate::__if_unsafe as if_unsafe;
}

mod macros;

/// Marks a function as a library/executable destructor. This uses OS-specific
/// linker sections to call a specific function at termination time.
///
/// Multiple shutdown functions are supported, but the invocation order is not
/// guaranteed.
///
/// `sys_common::at_exit` is usually a better solution for shutdown handling, as
/// it allows you to use `stdout` in your handlers.
///
/// ```rust
/// # #![cfg_attr(feature="used_linker", feature(used_with_arg))]
/// # extern crate dtor;
/// # use dtor::*;
/// # fn main() {}
///
/// #[dtor]
/// fn shutdown() {
///   /* ... */
/// }
/// ```
pub use dtor_proc_macro::dtor;
