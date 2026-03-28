use std::marker::PhantomData;

#[doc(hidden)]
pub mod __support {
    pub use crate::__in_section_parse as in_section_parse;
    pub use crate::__section_parse as section_parse;

    /// Define a link section.
    #[macro_export]
    macro_rules! __section_parse {
        (#[section(data)] $item:item) => {
            
        };
    }

    /// Export a symbol into a link section.
    #[macro_export]
    macro_rules! __in_section_parse {
        (#[in_section($name:path)] $item:item) => {
            
        };
    }
}

pub use ::link_section_proc_macro::in_section as in_section;
pub use ::link_section_proc_macro::section as section;

pub struct Section {
}

pub struct TypedSection<T> {
    _phantom: PhantomData<T>,
}
