use clitest_lib::clitest;

clitest!(basic, r#"
set RUSTFLAGS "";
cd "link_section/basic";
defer {
    $ cargo clean --quiet
}
$ cargo build --quiet
*
$ cargo run --quiet
! LINK_SECTION: Section { name: "__TEXT,LINK_SECTION", start: 0x100c600e8, end: 0x100c60110, byte_len: 40 }
! link_section_function
! TYPED_LINK_SECTION: TypedSection { name: "__DATA,TYPED_LINKIXOI_M", start: 0x100c74268, end: 0x100c74270, len: 2, stride: 4 }
! CODE_SECTION: TypedSection { name: "__DATA,FN_ARRAY", start: 0x100c74220, end: 0x100c74238, len: 3, stride: 8 }
! [0x100c600e8, 0x100c29868, 0x100c29890]
! f: 0x100c600e8
! f: 0x100c29868
! linked_function
! f: 0x100c29890
! linked_function_2
! DEBUGGABLES: [1, 2, 0x100c298b8]
"#);
