pub mod macros;

pub use __support::ctor_parse;

#[doc(hidden)]
#[allow(unused)]
pub mod __support {
    #[doc(hidden)]
    pub use crate::__ctor_parse as ctor_parse;
    pub use crate::__unify_features as unify_features;
    pub use crate::__ctor_entry as ctor_entry;
    pub use crate::__dtor_entry as dtor_entry;
    pub use crate::__if_unsafe as if_unsafe;
    pub use crate::__if_has_feature as if_has_feature;
    pub use crate::__ctor_link_section as ctor_link_section;
    pub use crate::__ctor_link_section_attr as ctor_link_section_attr;
}
